use axum::{
    Json,
    Router,
    extract::{Extension, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::get,
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter,
    Set,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    middleware::auth::optional_auth_middleware,
    services::auth::Claims,
    models::hackathon::*,
    models::team::TeamSummary,
    services::state::SharedState,
    utils::error::AppError,
};
use entity::{deadlines, hackathons, team_members, teams, tracks, users};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(list_hackathons).post(create_hackathon))
        .route("/:id", get(get_hackathon).put(update_hackathon).delete(delete_hackathon))
        .route("/:id/participants", get(get_hackathon_participants))
        .route("/:id/teams", get(get_hackathon_teams))
        .layer(middleware::from_fn(optional_auth_middleware))
}

async fn list_hackathons(
    State(state): State<SharedState>,
) -> Result<Json<HackathonListResponse>, AppError> {
    let hackathons = hackathons::Entity::find()
        .filter(hackathons::Column::IsPublished.eq(true))
        .filter(hackathons::Column::EventEnd.gte(Utc::now()))
        .all(&state.db)
        .await?;

    let summaries: Vec<HackathonSummary> = hackathons
        .into_iter()
        .map(|h| HackathonSummary {
            id: h.id.to_string(),
            title: h.title,
            banner_url: h.banner_url,
            location_type: h.location_type,
            registration_start: h.registration_start.to_string(),
            registration_end: h.registration_end.to_string(),
            event_start: h.event_start.to_string(),
            event_end: h.event_end.to_string(),
            participant_count: 0,
            team_count: 0,
        })
        .collect();

    let total = summaries.len() as i64;

    Ok(Json(HackathonListResponse {
        hackathons: summaries,
        total,
    }))
}

async fn create_hackathon(
    State(state): State<SharedState>,
    claims: Option<Extension<Claims>>,
    Json(req): Json<CreateHackathonRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let claims = claims
        .map(|Extension(claims)| claims)
        .ok_or(AppError::Unauthorized("Not authenticated".to_string()))?;

    let hackathon = hackathons::ActiveModel {
        title: Set(req.title),
        description: Set(req.description),
        location_type: Set(req.location_type),
        city: Set(req.city),
        venue: Set(req.venue),
        registration_start: Set(req.registration_start.fixed_offset()),
        registration_end: Set(req.registration_end.fixed_offset()),
        event_start: Set(req.event_start.fixed_offset()),
        event_end: Set(req.event_end.fixed_offset()),
        max_participants: Set(req.max_participants),
        organizer_id: Set(Some(claims.sub)),
        is_published: Set(false),
        ..Default::default()
    };

    let hackathon = hackathon.insert(&state.db).await?;

    for track_req in req.tracks {
        let track = tracks::ActiveModel {
            hackathon_id: Set(hackathon.id),
            name: Set(track_req.name),
            description: Set(track_req.description),
            prize_description: Set(track_req.prize_description),
            max_teams: Set(track_req.max_teams),
            ..Default::default()
        };
        track.insert(&state.db).await?;
    }

    for deadline_req in req.deadlines {
        let deadline = deadlines::ActiveModel {
            hackathon_id: Set(hackathon.id),
            name: Set(deadline_req.name),
            description: Set(deadline_req.description),
            deadline_at: Set(deadline_req.deadline_at.fixed_offset()),
            is_milestone: Set(deadline_req.is_milestone),
            ..Default::default()
        };
        deadline.insert(&state.db).await?;
    }

    let response = build_hackathon_response(&state, hackathon.id).await?;

    Ok((StatusCode::CREATED, Json(response)))
}

async fn get_hackathon(
    State(state): State<SharedState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Result<Json<HackathonResponse>, AppError> {
    let response = build_hackathon_response(&state, id).await?;
    Ok(Json(response))
}

async fn update_hackathon(
    State(state): State<SharedState>,
    claims: Option<Extension<Claims>>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
    Json(req): Json<UpdateHackathonRequest>,
) -> Result<Json<HackathonResponse>, AppError> {
    let claims = claims
        .map(|Extension(claims)| claims)
        .ok_or(AppError::Unauthorized("Not authenticated".to_string()))?;

    let hackathon = hackathons::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Hackathon not found".to_string()))?;

    if hackathon.organizer_id != Some(claims.sub) {
        return Err(AppError::Forbidden("Not authorized".to_string()));
    }

    let mut hackathon_active: hackathons::ActiveModel = hackathon.into();

    if let Some(title) = req.title {
        hackathon_active.title = Set(title);
    }
    if let Some(description) = req.description {
        hackathon_active.description = Set(Some(description));
    }
    if let Some(location_type) = req.location_type {
        hackathon_active.location_type = Set(location_type);
    }
    if let Some(city) = req.city {
        hackathon_active.city = Set(Some(city));
    }
    if let Some(venue) = req.venue {
        hackathon_active.venue = Set(Some(venue));
    }
    if let Some(is_published) = req.is_published {
        hackathon_active.is_published = Set(is_published);
    }

    let _hackathon = hackathon_active.update(&state.db).await?;

    let response = build_hackathon_response(&state, id).await?;
    Ok(Json(response))
}

async fn delete_hackathon(
    State(state): State<SharedState>,
    claims: Option<Extension<Claims>>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let claims = claims
        .map(|Extension(claims)| claims)
        .ok_or(AppError::Unauthorized("Not authenticated".to_string()))?;

    let hackathon = hackathons::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Hackathon not found".to_string()))?;

    if hackathon.organizer_id != Some(claims.sub) {
        return Err(AppError::Forbidden("Not authorized".to_string()));
    }

    hackathons::Entity::delete_by_id(id).exec(&state.db).await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn get_hackathon_participants(
    State(state): State<SharedState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Result<Json<Vec<UserInfo>>, AppError> {
    let _hackathon = hackathons::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Hackathon not found".to_string()))?;

    let participants = team_members::Entity::find()
        .inner_join(teams::Entity)
        .filter(teams::Column::HackathonId.eq(id))
        .inner_join(users::Entity)
        .all(&state.db)
        .await?;

    let user_ids: Vec<Uuid> = participants.iter().map(|p| p.user_id).collect();
    let users = users::Entity::find()
        .filter(users::Column::Id.is_in(user_ids))
        .all(&state.db)
        .await?;

    let result: Vec<UserInfo> = users
        .into_iter()
        .map(|u| UserInfo {
            id: u.id.to_string(),
            name: u.name,
            avatar_url: u.avatar_url,
        })
        .collect();

    Ok(Json(result))
}

async fn get_hackathon_teams(
    State(state): State<SharedState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Result<Json<Vec<TeamSummary>>, AppError> {
    let teams = teams::Entity::find()
        .filter(teams::Column::HackathonId.eq(id))
        .all(&state.db)
        .await?;

    let summaries: Vec<TeamSummary> = teams
        .into_iter()
        .map(|t| TeamSummary {
            id: t.id.to_string(),
            name: t.name,
            description: t.description,
            hackathon_id: t.hackathon_id.to_string(),
            track_name: None,
            status: t.status,
            member_count: 0,
        })
        .collect();

    Ok(Json(summaries))
}

async fn build_hackathon_response(
    state: &SharedState,
    id: Uuid,
) -> Result<HackathonResponse, AppError> {
    let hackathon = hackathons::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Hackathon not found".to_string()))?;

    let organizer = if let Some(org_id) = hackathon.organizer_id {
        users::Entity::find_by_id(org_id)
            .one(&state.db)
            .await?
            .map(|u| OrganizerResponse {
                id: u.id.to_string(),
                name: u.name,
                avatar_url: u.avatar_url,
            })
    } else {
        None
    };

    let tracks_data = tracks::Entity::find()
        .filter(tracks::Column::HackathonId.eq(id))
        .all(&state.db)
        .await?;

    let tracks: Vec<TrackResponse> = tracks_data
        .into_iter()
        .map(|t| TrackResponse {
            id: t.id.to_string(),
            name: t.name,
            description: t.description,
            prize_description: t.prize_description,
            max_teams: t.max_teams,
            team_count: 0,
        })
        .collect();

    let deadlines_data = deadlines::Entity::find()
        .filter(deadlines::Column::HackathonId.eq(id))
        .all(&state.db)
        .await?;

    let deadlines: Vec<DeadlineResponse> = deadlines_data
        .into_iter()
        .map(|d| DeadlineResponse {
            id: d.id.to_string(),
            name: d.name,
            description: d.description,
            deadline_at: d.deadline_at.to_string(),
            is_milestone: d.is_milestone,
        })
        .collect();

    let response = HackathonResponse {
        id: hackathon.id.to_string(),
        title: hackathon.title,
        description: hackathon.description,
        banner_url: hackathon.banner_url,
        location_type: hackathon.location_type,
        city: hackathon.city,
        venue: hackathon.venue,
        registration_start: hackathon.registration_start.to_string(),
        registration_end: hackathon.registration_end.to_string(),
        event_start: hackathon.event_start.to_string(),
        event_end: hackathon.event_end.to_string(),
        max_participants: hackathon.max_participants,
        organizer,
        is_published: hackathon.is_published,
        tracks,
        deadlines,
        participant_count: 0,
        team_count: 0,
    };

    Ok(response)
}

#[derive(Debug, serde::Serialize)]
struct UserInfo {
    id: String,
    name: String,
    avatar_url: Option<String>,
}
