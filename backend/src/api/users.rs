use axum::{
    Json,
    Router,
    extract::{Extension, State},
    middleware,
    routing::{delete, get, post},
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;
use validator::Validate;

use crate::{
    middleware::auth::auth_middleware,
    models::user::*,
    services::auth::Claims,
    services::state::SharedState,
    utils::error::AppError,
};
use entity::{skills, user_skills, users};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/me", get(get_me).patch(update_me))
        .route("/me/skills", get(get_my_skills).post(add_skill).delete(remove_skill))
        .route("/me/skills/:id", post(update_skill_level))
        .route("/skills", get(get_available_skills))
        .route("/:id", get(get_user_by_id))
        .layer(middleware::from_fn(auth_middleware))
}

async fn get_user_skills(
    state: &SharedState,
    user_id: Uuid,
) -> Result<Vec<SkillResponse>, AppError> {
    let user_skills_data = user_skills::Entity::find()
        .filter(user_skills::Column::UserId.eq(user_id))
        .inner_join(skills::Entity)
        .all(&state.db)
        .await?;

    let mut responses = Vec::new();
    for us in user_skills_data {
        let skill = skills::Entity::find_by_id(us.skill_id)
            .one(&state.db)
            .await?
            .ok_or(AppError::NotFound("Skill not found".to_string()))?;

        responses.push(SkillResponse {
            id: skill.id.to_string(),
            name: skill.name,
            category: skill.category,
            level: us.level,
        });
    }

    Ok(responses)
}

async fn build_user_response(
    state: &SharedState,
    user: users::Model,
) -> Result<UserProfileResponse, AppError> {
    let skills = get_user_skills(state, user.id).await?;

    Ok(UserProfileResponse {
        id: user.id.to_string(),
        email: user.email,
        name: user.name,
        avatar_url: user.avatar_url,
        bio: user.bio,
        github_url: user.github_url,
        telegram_username: user.telegram_username,
        is_verified: user.is_verified,
        created_at: user.created_at.to_string(),
        skills,
    })
}

async fn get_me(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<UserProfileResponse>, AppError> {
    let user = users::Entity::find_by_id(claims.sub)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("User not found".to_string()))?;

    let response = build_user_response(&state, user).await?;
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

    let response = build_user_response(&state, user).await?;
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

    let response = build_user_response(&state, user).await?;
    Ok(Json(response))
}

async fn get_my_skills(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<SkillResponse>>, AppError> {
    let skills = get_user_skills(&state, claims.sub).await?;
    Ok(Json(skills))
}

async fn add_skill(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<AddSkillRequest>,
) -> Result<Json<Vec<SkillResponse>>, AppError> {
    let skill_id = Uuid::parse_str(&req.skill_id)
        .map_err(|_| AppError::BadRequest("Invalid skill ID".to_string()))?;

    // Check if skill exists
    let _skill = skills::Entity::find_by_id(skill_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Skill not found".to_string()))?;

    // Check if user already has this skill
    let existing = user_skills::Entity::find()
        .filter(user_skills::Column::UserId.eq(claims.sub))
        .filter(user_skills::Column::SkillId.eq(skill_id))
        .one(&state.db)
        .await?;

    if existing.is_none() {
        let user_skill = user_skills::ActiveModel {
            user_id: Set(claims.sub),
            skill_id: Set(skill_id),
            level: Set(req.level.clamp(1, 5)),
            ..Default::default()
        };
        user_skill.insert(&state.db).await?;
    }

    let skills = get_user_skills(&state, claims.sub).await?;
    Ok(Json(skills))
}

async fn remove_skill(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<RemoveSkillRequest>,
) -> Result<Json<Vec<SkillResponse>>, AppError> {
    let skill_id = Uuid::parse_str(&req.skill_id)
        .map_err(|_| AppError::BadRequest("Invalid skill ID".to_string()))?;

    user_skills::Entity::delete_many()
        .filter(user_skills::Column::UserId.eq(claims.sub))
        .filter(user_skills::Column::SkillId.eq(skill_id))
        .exec(&state.db)
        .await?;

    let skills = get_user_skills(&state, claims.sub).await?;
    Ok(Json(skills))
}

async fn update_skill_level(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
    Json(req): Json<UpdateSkillLevelRequest>,
) -> Result<Json<Vec<SkillResponse>>, AppError> {
    let skill_id = id;

    let user_skill = user_skills::Entity::find()
        .filter(user_skills::Column::UserId.eq(claims.sub))
        .filter(user_skills::Column::SkillId.eq(skill_id))
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Skill not found for user".to_string()))?;

    let mut active: user_skills::ActiveModel = user_skill.into();
    active.level = Set(req.level.clamp(1, 5));
    active.update(&state.db).await?;

    let skills = get_user_skills(&state, claims.sub).await?;
    Ok(Json(skills))
}

async fn get_available_skills(
    State(state): State<SharedState>,
) -> Result<Json<Vec<AvailableSkillResponse>>, AppError> {
    let skills = skills::Entity::find()
        .all(&state.db)
        .await?;

    let response: Vec<AvailableSkillResponse> = skills
        .into_iter()
        .map(|s| AvailableSkillResponse {
            id: s.id.to_string(),
            name: s.name,
            category: s.category,
        })
        .collect();

    Ok(Json(response))
}
