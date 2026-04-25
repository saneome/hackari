use axum::{
    extract::{State, Path},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set};
use validator::Validate;
use uuid::Uuid;

use crate::{
    middleware::auth::auth_middleware,
    models::organizer::*,
    services::auth::Claims,
    services::state::SharedState,
    utils::error::AppError,
};
use entity::{hackathons, organizers, team_members, teams, users};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/me", post(create_organizer).get(get_my_organizer).put(update_organizer))
        .route("/:id", get(get_public_organizer))
        .route("/:id/hackathons", get(get_organizer_hackathons))
        .layer(middleware::from_fn(auth_middleware))
}

async fn create_organizer(
    State(state): State<SharedState>,
    claims: axum::extract::Extension<Claims>,
    Json(req): Json<CreateOrganizerRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Check if user already has an organizer profile
    let existing = organizers::Entity::find()
        .filter(organizers::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    if existing.is_some() {
        return Err(AppError::Conflict("Организатор уже существует".to_string()));
    }

    let user = users::Entity::find_by_id(claims.sub)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Пользователь не найден".to_string()))?;

    if user.organizer_terms_accepted_at.is_none() {
        return Err(AppError::Forbidden(
            "Сначала примите условия использования платформы".to_string(),
        ));
    }

    let organizer = organizers::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(claims.sub),
        name: Set(req.name),
        type_: Set(req.type_),
        description: Set(req.description),
        website_url: Set(req.website_url),
        logo_url: Set(req.logo_url),
        email: Set(req.email),
        social_links: Set(req.social_links),
        is_verified: Set(false),
        created_at: Set(chrono::Utc::now().into()),
        updated_at: Set(chrono::Utc::now().into()),
    };

    let organizer = organizer.insert(&state.db).await?;

    let response = OrganizerResponse {
        id: organizer.id.to_string(),
        user_id: organizer.user_id.to_string(),
        name: organizer.name,
        type_: organizer.type_,
        description: organizer.description,
        website_url: organizer.website_url,
        logo_url: organizer.logo_url,
        email: organizer.email,
        social_links: organizer.social_links,
        is_verified: organizer.is_verified,
        created_at: organizer.created_at.to_rfc3339(),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

async fn get_my_organizer(
    State(state): State<SharedState>,
    claims: axum::extract::Extension<Claims>,
) -> Result<Json<OrganizerResponse>, AppError> {
    let organizer = organizers::Entity::find()
        .filter(organizers::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Организатор не найден".to_string()))?;

    let response = OrganizerResponse {
        id: organizer.id.to_string(),
        user_id: organizer.user_id.to_string(),
        name: organizer.name,
        type_: organizer.type_,
        description: organizer.description,
        website_url: organizer.website_url,
        logo_url: organizer.logo_url,
        email: organizer.email,
        social_links: organizer.social_links,
        is_verified: organizer.is_verified,
        created_at: organizer.created_at.to_rfc3339(),
    };

    Ok(Json(response))
}

async fn update_organizer(
    State(state): State<SharedState>,
    claims: axum::extract::Extension<Claims>,
    Json(req): Json<UpdateOrganizerRequest>,
) -> Result<Json<OrganizerResponse>, AppError> {
    req.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;

    let organizer = organizers::Entity::find()
        .filter(organizers::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Организатор не найден".to_string()))?;

    let mut organizer_active: organizers::ActiveModel = organizer.into();

    if let Some(name) = req.name {
        organizer_active.name = Set(name);
    }
    if let Some(description) = req.description {
        organizer_active.description = Set(Some(description));
    }
    if let Some(website_url) = req.website_url {
        organizer_active.website_url = Set(Some(website_url));
    }
    if let Some(logo_url) = req.logo_url {
        organizer_active.logo_url = Set(Some(logo_url));
    }
    if let Some(email) = req.email {
        organizer_active.email = Set(email);
    }
    if let Some(social_links) = req.social_links {
        organizer_active.social_links = Set(Some(social_links));
    }

    organizer_active.updated_at = Set(chrono::Utc::now().into());

    let organizer = organizer_active.update(&state.db).await?;

    let response = OrganizerResponse {
        id: organizer.id.to_string(),
        user_id: organizer.user_id.to_string(),
        name: organizer.name,
        type_: organizer.type_,
        description: organizer.description,
        website_url: organizer.website_url,
        logo_url: organizer.logo_url,
        email: organizer.email,
        social_links: organizer.social_links,
        is_verified: organizer.is_verified,
        created_at: organizer.created_at.to_rfc3339(),
    };

    Ok(Json(response))
}

async fn get_public_organizer(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<OrganizerPublicResponse>, AppError> {
    let organizer = organizers::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Организатор не найден".to_string()))?;

    let response = OrganizerPublicResponse {
        id: organizer.id.to_string(),
        name: organizer.name,
        type_: organizer.type_,
        description: organizer.description,
        website_url: organizer.website_url,
        logo_url: organizer.logo_url,
        social_links: organizer.social_links,
        is_verified: organizer.is_verified,
    };

    Ok(Json(response))
}

async fn get_organizer_hackathons(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<HackathonSummary>>, AppError> {
    let organizer = organizers::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Организатор не найден".to_string()))?;

    let hackathons = hackathons::Entity::find()
        .filter(hackathons::Column::OrganizerId.eq(organizer.id))
        .all(&state.db)
        .await?;

    let mut summaries: Vec<HackathonSummary> = Vec::with_capacity(hackathons.len());

    for h in hackathons {
        let team_count = teams::Entity::find()
            .filter(teams::Column::HackathonId.eq(h.id))
            .count(&state.db)
            .await? as i64;

        let participant_count = team_members::Entity::find()
            .inner_join(teams::Entity)
            .filter(teams::Column::HackathonId.eq(h.id))
            .count(&state.db)
            .await? as i64;

        summaries.push(HackathonSummary {
            id: h.id.to_string(),
            title: h.title,
            banner_url: h.banner_url,
            event_start: h.event_start.to_rfc3339(),
            event_end: h.event_end.to_rfc3339(),
            location_type: h.location_type,
            is_published: h.is_published,
            status: h.status,
            participant_count,
            team_count,
        });
    }

    Ok(Json(summaries))
}
