use axum::{
    Json,
    Router,
    extract::{Extension, Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    middleware::auth::auth_middleware,
    models::invitation::*,
    models::team::TeamResponse,
    services::{
        auth::{Claims, generate_session_token},
        state::SharedState,
        websocket::{WsMessage, broadcast_message},
    },
    utils::error::AppError,
};
use entity::{hackathons, invitations, team_members, teams, users};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(list_invitations))
        .route("/", post(create_invitation))
        .route("/:id/accept", post(accept_invitation))
        .route("/:id/decline", post(decline_invitation))
        .route("/pending", get(get_pending_invitations))
        .layer(middleware::from_fn(auth_middleware))
}

async fn list_invitations(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<InvitationListResponse>, AppError> {
    let invitations = invitations::Entity::find()
        .filter(invitations::Column::InvitedBy.eq(claims.sub))
        .all(&state.db)
        .await?;

    let total = invitations.len() as i64;

    let mut invitation_responses = Vec::new();
    for invitation in invitations {
        let team = teams::Entity::find_by_id(invitation.team_id)
            .one(&state.db)
            .await?;

        let inviter = users::Entity::find_by_id(invitation.invited_by)
            .one(&state.db)
            .await?;

        if let (Some(team), Some(inviter)) = (team, inviter) {
            invitation_responses.push(InvitationResponse {
                id: invitation.id.to_string(),
                team_id: team.id.to_string(),
                team_name: team.name,
                invited_by: UserInfo {
                    id: inviter.id.to_string(),
                    name: inviter.name,
                    avatar_url: inviter.avatar_url,
                },
                invited_email: invitation.invited_email,
                status: invitation.status,
                expires_at: invitation.expires_at.to_string(),
                created_at: invitation.created_at.to_string(),
            });
        }
    }

    Ok(Json(InvitationListResponse {
        invitations: invitation_responses,
        total,
    }))
}

async fn create_invitation(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
    Path(team_id): Path<Uuid>,
    Json(req): Json<CreateInvitationRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let team = teams::Entity::find_by_id(team_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Team not found".to_string()))?;

    let membership = team_members::Entity::find()
        .filter(team_members::Column::TeamId.eq(team_id))
        .filter(team_members::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    if membership.is_none() {
        return Err(AppError::Forbidden("Not a team member".to_string()));
    }

    let invited_user = users::Entity::find()
        .filter(users::Column::Email.eq(&req.email))
        .one(&state.db)
        .await?;

    if let Some(user) = &invited_user {
        let existing_membership = team_members::Entity::find()
            .inner_join(teams::Entity)
            .filter(teams::Column::HackathonId.eq(team.hackathon_id))
            .filter(team_members::Column::UserId.eq(user.id))
            .one(&state.db)
            .await?;

        if existing_membership.is_some() {
            return Err(AppError::Conflict("User already in a team for this hackathon".to_string()));
        }
    }

    let existing_invite = invitations::Entity::find()
        .filter(invitations::Column::TeamId.eq(team_id))
        .filter(invitations::Column::InvitedEmail.eq(&req.email))
        .filter(invitations::Column::Status.eq("pending"))
        .one(&state.db)
        .await?;

    if existing_invite.is_some() {
        return Err(AppError::Conflict("Invitation already sent".to_string()));
    }

    let token = generate_session_token();

    let invitation = invitations::ActiveModel {
        team_id: Set(team_id),
        invited_by: Set(claims.sub),
        invited_email: Set(req.email.clone()),
        invited_user_id: Set(invited_user.as_ref().map(|u| u.id)),
        token: Set(token.clone()),
        expires_at: Set((Utc::now() + chrono::Duration::days(7)).fixed_offset()),
        ..Default::default()
    };

    let invitation = invitation.insert(&state.db).await?;

    let inviter = users::Entity::find_by_id(claims.sub)
        .one(&state.db)
        .await?;

    if let Some(inviter) = &inviter {
        broadcast_message(
            &state,
            WsMessage::TeamInvite {
                team_id: team_id.to_string(),
                team_name: team.name.clone(),
                invited_by: inviter.name.clone(),
            },
        );
    }

    let response = InvitationResponse {
        id: invitation.id.to_string(),
        team_id: team.id.to_string(),
        team_name: team.name,
        invited_by: UserInfo {
            id: inviter.as_ref().map(|u| u.id.to_string()).unwrap_or_default(),
            name: inviter.as_ref().map(|u| u.name.clone()).unwrap_or_default(),
            avatar_url: inviter.as_ref().and_then(|u| u.avatar_url.clone()),
        },
        invited_email: req.email,
        status: invitation.status,
        expires_at: invitation.expires_at.to_string(),
        created_at: invitation.created_at.to_string(),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

async fn accept_invitation(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<TeamResponse>, AppError> {
    let invitation = invitations::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Invitation not found".to_string()))?;

    if invitation.status != "pending" {
        return Err(AppError::Conflict("Invitation already processed".to_string()));
    }

    if Utc::now() > invitation.expires_at {
        return Err(AppError::BadRequest("Invitation expired".to_string()));
    }

    if let Some(invited_user_id) = invitation.invited_user_id {
        if invited_user_id != claims.sub {
            return Err(AppError::Forbidden("Not authorized".to_string()));
        }
    }

    let team = teams::Entity::find_by_id(invitation.team_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Team not found".to_string()))?;

    let existing_membership = team_members::Entity::find()
        .inner_join(teams::Entity)
        .filter(teams::Column::HackathonId.eq(team.hackathon_id))
        .filter(team_members::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    if existing_membership.is_some() {
        return Err(AppError::Conflict("Already in a team for this hackathon".to_string()));
    }

    let team_id = invitation.team_id;
    let mut invitation_active: invitations::ActiveModel = invitation.into();
    invitation_active.status = Set("accepted".to_string());
    invitation_active.update(&state.db).await?;

    let member = team_members::ActiveModel {
        team_id: Set(team_id),
        user_id: Set(claims.sub),
        role: Set("member".to_string()),
        ..Default::default()
    };
    member.insert(&state.db).await?;

    let response = build_team_response(&state, team_id).await?;
    Ok(Json(response))
}

async fn decline_invitation(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let invitation = invitations::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Invitation not found".to_string()))?;

    if let Some(invited_user_id) = invitation.invited_user_id {
        if invited_user_id != claims.sub {
            return Err(AppError::Forbidden("Not authorized".to_string()));
        }
    }

    let mut invitation_active: invitations::ActiveModel = invitation.into();
    invitation_active.status = Set("declined".to_string());
    invitation_active.update(&state.db).await?;

    Ok(StatusCode::OK)
}

async fn get_pending_invitations(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<PendingInvitationResponse>>, AppError> {
    let invitations = invitations::Entity::find()
        .filter(invitations::Column::InvitedUserId.eq(claims.sub))
        .filter(invitations::Column::Status.eq("pending"))
        .filter(invitations::Column::ExpiresAt.gt(Utc::now()))
        .all(&state.db)
        .await?;

    let mut responses = Vec::new();
    for invitation in invitations {
        let team = teams::Entity::find_by_id(invitation.team_id)
            .one(&state.db)
            .await?;

        let inviter = users::Entity::find_by_id(invitation.invited_by)
            .one(&state.db)
            .await?;

        if let (Some(team), Some(inviter)) = (team, inviter) {
            let hackathon_name = hackathons::Entity::find_by_id(team.hackathon_id)
                .one(&state.db)
                .await?
                .map(|h| h.title)
                .unwrap_or_default();

            responses.push(PendingInvitationResponse {
                id: invitation.id.to_string(),
                team: TeamInfo {
                    id: team.id.to_string(),
                    name: team.name,
                    hackathon_name,
                },
                invited_by: UserInfo {
                    id: inviter.id.to_string(),
                    name: inviter.name,
                    avatar_url: inviter.avatar_url,
                },
                expires_at: invitation.expires_at.to_string(),
            });
        }
    }

    Ok(Json(responses))
}

async fn build_team_response(
    state: &SharedState,
    id: Uuid,
) -> Result<crate::models::team::TeamResponse, AppError> {
    use crate::models::team::*;
    use entity::{teams, team_members, users, tracks};

    let team = teams::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Team not found".to_string()))?;

    let track = if let Some(track_id) = team.track_id {
        tracks::Entity::find_by_id(track_id)
            .one(&state.db)
            .await?
            .map(|t| TrackInfo {
                id: t.id.to_string(),
                name: t.name,
            })
    } else {
        None
    };

    let members = team_members::Entity::find()
        .filter(team_members::Column::TeamId.eq(id))
        .all(&state.db)
        .await?;

    let mut member_responses = Vec::new();
    for member in members {
        let user = users::Entity::find_by_id(member.user_id)
            .one(&state.db)
            .await?;

        if let Some(user) = user {
            member_responses.push(TeamMemberResponse {
                id: member.id.to_string(),
                user_id: member.user_id.to_string(),
                name: user.name,
                avatar_url: user.avatar_url,
                role: member.role,
                joined_at: member.joined_at.to_string(),
            });
        }
    }

    let response = TeamResponse {
        id: team.id.to_string(),
        name: team.name,
        description: team.description,
        hackathon_id: team.hackathon_id.to_string(),
        track,
        repo_url: team.repo_url,
        demo_url: team.demo_url,
        presentation_url: team.presentation_url,
        status: team.status,
        members: member_responses,
        created_at: team.created_at.to_string(),
    };

    Ok(response)
}
