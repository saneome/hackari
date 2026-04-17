use axum::{
    Json,
    Router,
    extract::{Extension, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use validator::Validate;

use crate::{
    middleware::auth::optional_auth_middleware,
    models::auth::*,
    services::{
        auth::*,
        state::SharedState,
    },
    utils::error::AppError,
};
use entity::{sessions, users};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/refresh", post(refresh_token))
        .route("/me", get(get_current_user).layer(middleware::from_fn(optional_auth_middleware)))
}

async fn register(
    State(state): State<SharedState>,
    jar: CookieJar,
    Json(req): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let existing = users::Entity::find()
        .filter(users::Column::Email.eq(&req.email))
        .one(&state.db)
        .await?;

    if existing.is_some() {
        return Err(AppError::Conflict("Email already registered".to_string()));
    }

    let password_hash = hash_password(&req.password)?;

    let user = users::ActiveModel {
        email: Set(req.email.clone()),
        password_hash: Set(password_hash),
        name: Set(req.name),
        ..Default::default()
    };

    let user = user.insert(&state.db).await?;

    let access_token = generate_access_token(user.id, &user.email)?;
    let refresh_token = generate_refresh_token(user.id, &user.email)?;

    let session_token = generate_session_token();
    let session_hash = hash_password(&session_token).unwrap_or_default();

    let session = sessions::ActiveModel {
        user_id: Set(user.id),
        token_hash: Set(session_hash),
        expires_at: Set((Utc::now() + chrono::Duration::days(7)).fixed_offset()),
        ..Default::default()
    };
    session.insert(&state.db).await?;

    let access_cookie = Cookie::build(("access_token", access_token.clone()))
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(time::Duration::minutes(15))
        .build();

    let refresh_cookie = Cookie::build(("refresh_token", refresh_token.clone()))
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(time::Duration::days(7))
        .build();

    let jar = jar.add(access_cookie).add(refresh_cookie);

    let user_response = UserResponse {
        id: user.id.to_string(),
        email: user.email,
        name: user.name,
        avatar_url: user.avatar_url,
        bio: user.bio,
        github_url: user.github_url,
        telegram_username: user.telegram_username,
        is_verified: user.is_verified,
    };

    let response = AuthResponse {
        user: user_response,
        access_token,
        refresh_token,
    };

    Ok((StatusCode::CREATED, jar, Json(response)))
}

async fn login(
    State(state): State<SharedState>,
    jar: CookieJar,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let user = users::Entity::find()
        .filter(users::Column::Email.eq(&req.email))
        .one(&state.db)
        .await?
        .ok_or(AppError::Unauthorized("Invalid credentials".to_string()))?;

    let valid = verify_password(&req.password, &user.password_hash)?;
    if !valid {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    let access_token = generate_access_token(user.id, &user.email)?;
    let refresh_token = generate_refresh_token(user.id, &user.email)?;

    let session_token = generate_session_token();
    let session_hash = hash_password(&session_token).unwrap_or_default();

    let session = sessions::ActiveModel {
        user_id: Set(user.id),
        token_hash: Set(session_hash),
        expires_at: Set((Utc::now() + chrono::Duration::days(7)).fixed_offset()),
        ..Default::default()
    };
    session.insert(&state.db).await?;

    let access_cookie = Cookie::build(("access_token", access_token.clone()))
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(time::Duration::minutes(15))
        .build();

    let refresh_cookie = Cookie::build(("refresh_token", refresh_token.clone()))
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(time::Duration::days(7))
        .build();

    let jar = jar.add(access_cookie).add(refresh_cookie);

    let user_response = UserResponse {
        id: user.id.to_string(),
        email: user.email,
        name: user.name,
        avatar_url: user.avatar_url,
        bio: user.bio,
        github_url: user.github_url,
        telegram_username: user.telegram_username,
        is_verified: user.is_verified,
    };

    let response = AuthResponse {
        user: user_response,
        access_token,
        refresh_token,
    };

    Ok((StatusCode::OK, jar, Json(response)))
}

async fn logout(
    State(_state): State<SharedState>,
    jar: CookieJar,
) -> Result<impl IntoResponse, AppError> {
    let mut jar = jar;

    let access_cookie = Cookie::build(("access_token", ""))
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(time::Duration::ZERO)
        .build();

    let refresh_cookie = Cookie::build(("refresh_token", ""))
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(time::Duration::ZERO)
        .build();

    jar = jar.add(access_cookie).add(refresh_cookie);

    Ok((StatusCode::OK, jar, Json(serde_json::json!({"message": "Logged out"}))))
}

async fn refresh_token(
    State(state): State<SharedState>,
    jar: CookieJar,
    Json(req): Json<RefreshTokenRequest>,
) -> Result<impl IntoResponse, AppError> {
    let claims = decode_token(&req.refresh_token)?;

    if claims.token_type != "refresh" {
        return Err(AppError::Unauthorized("Invalid token type".to_string()));
    }

    let user_id = claims.sub;
    let user = users::Entity::find_by_id(user_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::Unauthorized("User not found".to_string()))?;

    let access_token = generate_access_token(user.id, &user.email)?;
    let refresh_token = generate_refresh_token(user.id, &user.email)?;

    let access_cookie = Cookie::build(("access_token", access_token.clone()))
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(time::Duration::minutes(15))
        .build();

    let refresh_cookie = Cookie::build(("refresh_token", refresh_token.clone()))
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(time::Duration::days(7))
        .build();

    let jar = jar.add(access_cookie).add(refresh_cookie);

    let response = RefreshTokenResponse {
        access_token,
        refresh_token,
    };

    Ok((StatusCode::OK, jar, Json(response)))
}

async fn get_current_user(
    State(state): State<SharedState>,
    claims: Option<Extension<crate::services::auth::Claims>>,
) -> Result<Json<UserResponse>, AppError> {
    if let Some(Extension(claims)) = claims {
        let user = users::Entity::find_by_id(claims.sub)
            .one(&state.db)
            .await?
            .ok_or(AppError::NotFound("User not found".to_string()))?;

        let user_response = UserResponse {
            id: user.id.to_string(),
            email: user.email,
            name: user.name,
            avatar_url: user.avatar_url,
            bio: user.bio,
            github_url: user.github_url,
            telegram_username: user.telegram_username,
            is_verified: user.is_verified,
        };

        return Ok(Json(user_response));
    }

    Err(AppError::Unauthorized("Not authenticated".to_string()))
}
