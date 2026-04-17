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
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    middleware::auth::auth_middleware,
    models::team::*,
    services::{
        auth::Claims,
        state::SharedState,
        websocket::{WsMessage, broadcast_message},
    },
    utils::error::AppError,
};
use entity::{hackathons, submissions, team_members, teams, tracks, users};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/", post(create_team))
        .route("/:id", get(get_team).put(update_team).delete(delete_team))
        .route("/:id/join", post(join_team))
        .route("/:id/leave", post(leave_team))
        .route("/:id/submission", get(get_submission).post(create_submission).put(update_submission))
        .layer(middleware::from_fn(auth_middleware))
}

async fn create_team(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
    Path(hackathon_id): Path<Uuid>,
    Json(req): Json<CreateTeamRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let hackathon = hackathons::Entity::find_by_id(hackathon_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Hackathon not found".to_string()))?;

    if Utc::now() > hackathon.registration_end {
        return Err(AppError::BadRequest("Registration is closed".to_string()));
    }

    let existing_membership = team_members::Entity::find()
        .inner_join(teams::Entity)
        .filter(teams::Column::HackathonId.eq(hackathon_id))
        .filter(team_members::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    if existing_membership.is_some() {
        return Err(AppError::Conflict("Already in a team for this hackathon".to_string()));
    }

    let track_id = req.track_id.map(|id| Uuid::parse_str(&id).ok()).flatten();

    let team = teams::ActiveModel {
        hackathon_id: Set(hackathon_id),
        track_id: Set(track_id),
        name: Set(req.name),
        description: Set(req.description),
        status: Set("forming".to_string()),
        ..Default::default()
    };

    let team = team.insert(&state.db).await?;

    let member = team_members::ActiveModel {
        team_id: Set(team.id),
        user_id: Set(claims.sub),
        role: Set("leader".to_string()),
        ..Default::default()
    };
    member.insert(&state.db).await?;

    let response = build_team_response(&state, team.id).await?;

    Ok((StatusCode::CREATED, Json(response)))
}

async fn get_team(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<TeamResponse>, AppError> {
    let response = build_team_response(&state, id).await?;
    Ok(Json(response))
}

async fn update_team(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTeamRequest>,
) -> Result<Json<TeamResponse>, AppError> {
    let team = teams::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Team not found".to_string()))?;

    let membership = team_members::Entity::find()
        .filter(team_members::Column::TeamId.eq(id))
        .filter(team_members::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    if membership.is_none() || membership.as_ref().unwrap().role != "leader" {
        return Err(AppError::Forbidden("Only team leader can update".to_string()));
    }

    let mut team_active: teams::ActiveModel = team.into();

    if let Some(name) = req.name {
        team_active.name = Set(name);
    }
    if let Some(description) = req.description {
        team_active.description = Set(Some(description));
    }
    if let Some(track_id) = req.track_id {
        if let Ok(id) = Uuid::parse_str(&track_id) {
            team_active.track_id = Set(Some(id));
        }
    }
    if let Some(repo_url) = req.repo_url {
        team_active.repo_url = Set(Some(repo_url));
    }
    if let Some(demo_url) = req.demo_url {
        team_active.demo_url = Set(Some(demo_url));
    }
    if let Some(presentation_url) = req.presentation_url {
        team_active.presentation_url = Set(Some(presentation_url));
    }

    let _team = team_active.update(&state.db).await?;

    let response = build_team_response(&state, id).await?;
    Ok(Json(response))
}

async fn delete_team(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let _team = teams::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Team not found".to_string()))?;

    let membership = team_members::Entity::find()
        .filter(team_members::Column::TeamId.eq(id))
        .filter(team_members::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    if membership.is_none() || membership.as_ref().unwrap().role != "leader" {
        return Err(AppError::Forbidden("Only team leader can delete".to_string()));
    }

    teams::Entity::delete_by_id(id).exec(&state.db).await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn join_team(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(_req): Json<JoinTeamRequest>,
) -> Result<Json<TeamResponse>, AppError> {
    let team = teams::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Team not found".to_string()))?;

    let hackathon = hackathons::Entity::find_by_id(team.hackathon_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Hackathon not found".to_string()))?;

    if Utc::now() > hackathon.event_end {
        return Err(AppError::BadRequest("Hackathon has ended".to_string()));
    }

    let existing_membership = team_members::Entity::find()
        .inner_join(teams::Entity)
        .filter(teams::Column::HackathonId.eq(team.hackathon_id))
        .filter(team_members::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    if existing_membership.is_some() {
        return Err(AppError::Conflict("Already in a team for this hackathon".to_string()));
    }

    let member_count = team_members::Entity::find()
        .filter(team_members::Column::TeamId.eq(id))
        .count(&state.db)
        .await?;

    if member_count >= 5 {
        return Err(AppError::Conflict("Team is full".to_string()));
    }

    let member = team_members::ActiveModel {
        team_id: Set(id),
        user_id: Set(claims.sub),
        role: Set("member".to_string()),
        ..Default::default()
    };
    member.insert(&state.db).await?;

    let user = users::Entity::find_by_id(claims.sub)
        .one(&state.db)
        .await?;

    if let Some(user) = user {
        broadcast_message(
            &state,
            WsMessage::TeamJoin {
                team_id: id.to_string(),
                user_name: user.name,
            },
        );
    }

    let response = build_team_response(&state, id).await?;
    Ok(Json(response))
}

async fn leave_team(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<LeaveTeamResponse>, AppError> {
    let membership = team_members::Entity::find()
        .filter(team_members::Column::TeamId.eq(id))
        .filter(team_members::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Not a member of this team".to_string()))?;

    let is_leader = membership.role == "leader";
    let member_count = team_members::Entity::find()
        .filter(team_members::Column::TeamId.eq(id))
        .count(&state.db)
        .await?;

    team_members::Entity::delete_by_id(membership.id)
        .exec(&state.db)
        .await?;

    let user = users::Entity::find_by_id(claims.sub)
        .one(&state.db)
        .await?;

    if let Some(user) = user {
        broadcast_message(
            &state,
            WsMessage::TeamLeave {
                team_id: id.to_string(),
                user_name: user.name,
            },
        );
    }

    let mut response = LeaveTeamResponse {
        success: true,
        new_leader: None,
        team_disbanded: false,
    };

    if is_leader && member_count > 1 {
        let new_leader = team_members::Entity::find()
            .filter(team_members::Column::TeamId.eq(id))
            .one(&state.db)
            .await?;

        if let Some(new_leader) = new_leader {
            let mut new_leader_active: team_members::ActiveModel = new_leader.into();
            new_leader_active.role = Set("leader".to_string());
            let leader = new_leader_active.update(&state.db).await?;
            response.new_leader = Some(leader.user_id.to_string());
        }
    } else if member_count == 1 {
        teams::Entity::delete_by_id(id).exec(&state.db).await?;
        response.team_disbanded = true;
    }

    Ok(Json(response))
}

async fn get_submission(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Option<SubmissionResponse>>, AppError> {
    let submission = submissions::Entity::find()
        .filter(submissions::Column::TeamId.eq(id))
        .one(&state.db)
        .await?;

    let response = submission.map(|s| SubmissionResponse {
        id: s.id.to_string(),
        title: s.title,
        description: s.description,
        repo_url: s.repo_url,
        demo_url: s.demo_url,
        status: s.status,
        submitted_at: s.submitted_at.map(|d| d.to_string()),
    });

    Ok(Json(response))
}

async fn create_submission(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(req): Json<CreateSubmissionRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let membership = team_members::Entity::find()
        .filter(team_members::Column::TeamId.eq(id))
        .filter(team_members::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    if membership.is_none() {
        return Err(AppError::Forbidden("Not a team member".to_string()));
    }

    let existing = submissions::Entity::find()
        .filter(submissions::Column::TeamId.eq(id))
        .one(&state.db)
        .await?;

    if existing.is_some() {
        return Err(AppError::Conflict("Submission already exists".to_string()));
    }

    let submission = submissions::ActiveModel {
        team_id: Set(id),
        title: Set(req.title),
        description: Set(req.description),
        repo_url: Set(req.repo_url),
        demo_url: Set(req.demo_url),
        status: Set("draft".to_string()),
        ..Default::default()
    };

    let submission = submission.insert(&state.db).await?;

    let response = SubmissionResponse {
        id: submission.id.to_string(),
        title: submission.title,
        description: submission.description,
        repo_url: submission.repo_url,
        demo_url: submission.demo_url,
        status: submission.status,
        submitted_at: submission.submitted_at.map(|d| d.to_string()),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

async fn update_submission(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateSubmissionRequest>,
) -> Result<Json<SubmissionResponse>, AppError> {
    let membership = team_members::Entity::find()
        .filter(team_members::Column::TeamId.eq(id))
        .filter(team_members::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    if membership.is_none() {
        return Err(AppError::Forbidden("Not a team member".to_string()));
    }

    let submission = submissions::Entity::find()
        .filter(submissions::Column::TeamId.eq(id))
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Submission not found".to_string()))?;

    let mut submission_active: submissions::ActiveModel = submission.into();

    if let Some(title) = req.title {
        submission_active.title = Set(title);
    }
    if let Some(description) = req.description {
        submission_active.description = Set(Some(description));
    }
    if let Some(repo_url) = req.repo_url {
        submission_active.repo_url = Set(Some(repo_url));
    }
    if let Some(demo_url) = req.demo_url {
        submission_active.demo_url = Set(Some(demo_url));
    }

    let submission = submission_active.update(&state.db).await?;

    let response = SubmissionResponse {
        id: submission.id.to_string(),
        title: submission.title,
        description: submission.description,
        repo_url: submission.repo_url,
        demo_url: submission.demo_url,
        status: submission.status,
        submitted_at: submission.submitted_at.map(|d| d.to_string()),
    };

    Ok(Json(response))
}

async fn build_team_response(
    state: &SharedState,
    id: Uuid,
) -> Result<TeamResponse, AppError> {
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
