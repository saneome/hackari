use axum::{
    Json,
    Router,
    extract::{Extension, State},
    middleware,
    routing::get,
};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use validator::Validate;

use crate::{
    middleware::auth::auth_middleware,
    models::user::*,
    services::auth::Claims,
    services::state::SharedState,
    utils::error::AppError,
};
use entity::users;

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/me", get(get_me).patch(update_me))
        .route("/:id", get(get_user_by_id))
        .layer(middleware::from_fn(auth_middleware))
}

async fn get_me(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<UserProfileResponse>, AppError> {
    let user = users::Entity::find_by_id(claims.sub)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("User not found".to_string()))?;

    let response = UserProfileResponse {
        id: user.id.to_string(),
        email: user.email,
        name: user.name,
        avatar_url: user.avatar_url,
        bio: user.bio,
        github_url: user.github_url,
        telegram_username: user.telegram_username,
        is_verified: user.is_verified,
        created_at: user.created_at.to_string(),
    };

    Ok(Json(response))
}

async fn update_me(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<Json<UserProfileResponse>, AppError> {
    req.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let user = users::Entity::find_by_id(claims.sub)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("User not found".to_string()))?;

    let mut user_active: users::ActiveModel = user.into();

    if let Some(name) = req.name {
        user_active.name = Set(name);
    }
    if let Some(bio) = req.bio {
        user_active.bio = Set(Some(bio));
    }
    if let Some(github_url) = req.github_url {
        user_active.github_url = Set(Some(github_url));
    }
    if let Some(telegram_username) = req.telegram_username {
        user_active.telegram_username = Set(Some(telegram_username));
    }

    let user = user_active.update(&state.db).await?;

    let response = UserProfileResponse {
        id: user.id.to_string(),
        email: user.email,
        name: user.name,
        avatar_url: user.avatar_url,
        bio: user.bio,
        github_url: user.github_url,
        telegram_username: user.telegram_username,
        is_verified: user.is_verified,
        created_at: user.created_at.to_string(),
    };

    Ok(Json(response))
}

async fn get_user_by_id(
    State(state): State<SharedState>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> Result<Json<UserProfileResponse>, AppError> {
    let user = users::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("User not found".to_string()))?;

    let response = UserProfileResponse {
        id: user.id.to_string(),
        email: user.email,
        name: user.name,
        avatar_url: user.avatar_url,
        bio: user.bio,
        github_url: user.github_url,
        telegram_username: user.telegram_username,
        is_verified: user.is_verified,
        created_at: user.created_at.to_string(),
    };

    Ok(Json(response))
}
