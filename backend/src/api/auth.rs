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
use redis::AsyncCommands;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
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
use entity::{sessions, users, reset_code};
use rand::RngCore;

#[derive(Serialize, Deserialize)]
struct RedisResetCode {
    code: String,
    user_id: String,
    created_at: String,
}

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/refresh", post(refresh_token))
        .route("/me", get(get_current_user).layer(middleware::from_fn(optional_auth_middleware)))
        .route("/password-reset/request", post(request_reset))
        .route("/password-reset/verify", post(verify_reset_code))
        .route("/password-reset/reset", post(reset_password))
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
) -> Result<impl IntoResponse, AppError> {
    let refresh_token = jar
        .get("refresh_token")
        .map(|c| c.value().to_string())
        .ok_or(AppError::Unauthorized("No refresh token".to_string()))?;

    let claims = decode_token(&refresh_token)?;

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

// Password Reset Endpoints
async fn request_reset(
    State(state): State<SharedState>,
    Json(req): Json<RequestResetRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Find user by email
    let user = users::Entity::find()
        .filter(users::Column::Email.eq(&req.email))
        .one(&state.db)
        .await?;

    // Don't reveal if email exists for security
    let Some(user) = user else {
        return Ok((StatusCode::OK, Json(ResetCodeResponse {
            message: "If the email exists, a reset code has been sent".to_string(),
        })));
    };

    // Generate 6-digit code
    let mut code_bytes = [0u8; 3];
    rand::thread_rng().fill_bytes(&mut code_bytes);
    let code = format!("{:06}", u32::from_le_bytes([code_bytes[0], code_bytes[1], code_bytes[2], 0]) % 1000000);

    // Save code to Redis with 15 min TTL
    let redis_key = format!("reset_code:{}", req.email);
    let reset_data = RedisResetCode {
        code: code.clone(),
        user_id: user.id.to_string(),
        created_at: Utc::now().to_rfc3339(),
    };
    let redis_json = serde_json::to_string(&reset_data)?;
    let mut redis_conn = state.redis.clone();
    let _: () = redis_conn.set_ex(&redis_key, redis_json, 900).await?;

    // Send email
    match state.email_service.send_reset_code(&req.email, &code).await {
        Ok(_) => {
            Ok((StatusCode::OK, Json(ResetCodeResponse {
                message: "If the email exists, a reset code has been sent".to_string(),
            })))
        }
        Err(e) => {
            tracing::error!("Failed to send reset email: {:?}", e);
            // Still return success to not leak email existence
            Ok((StatusCode::OK, Json(ResetCodeResponse {
                message: "If the email exists, a reset code has been sent".to_string(),
            })))
        }
    }
}

async fn verify_reset_code(
    State(state): State<SharedState>,
    Json(req): Json<VerifyResetCodeRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate()
    .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let redis_key = format!("reset_code:{}", req.email);
    let mut redis_conn = state.redis.clone();
    let redis_data: Option<String> = redis_conn.get(&redis_key).await?;

    let Some(redis_json) = redis_data else {
        return Err(AppError::BadRequest("Invalid or expired code".to_string()));
    };

    let reset_data: RedisResetCode = serde_json::from_str(&redis_json)
        .map_err(|_| AppError::Internal("Failed to parse reset code".to_string()))?;

    if reset_data.code != req.code {
        return Err(AppError::BadRequest("Invalid or expired code".to_string()));
    }

    Ok((StatusCode::OK, Json(ResetCodeResponse {
        message: "Code is valid".to_string(),
    })))
}




async fn reset_password(
    State(state): State<SharedState>,
    Json(req): Json<ResetPasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate()
    .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Find user by email
    let user = users::Entity::find()
    .filter(users::Column::Email.eq(&req.email))
    .one(&state.db)
    .await?
    .ok_or_else(|| AppError::BadRequest("Invalid email".to_string()))?;

    // Check and delete code from Redis (atomic operation)
    let redis_key = format!("reset_code:{}", req.email);
    let mut redis_conn = state.redis.clone();
    let redis_data: Option<String> = redis_conn.get(&redis_key).await?;

    let Some(redis_json) = redis_data else {
        return Err(AppError::BadRequest("Invalid or expired code".to_string()));
    };

    let reset_data: RedisResetCode = serde_json::from_str(&redis_json)
        .map_err(|_| AppError::Internal("Failed to parse reset code".to_string()))?;

    if reset_data.code != req.code {
        return Err(AppError::BadRequest("Invalid or expired code".to_string()));
    }

    // Delete code from Redis (mark as used)
    let _: () = redis_conn.del(&redis_key).await?;

    // Hash new password
    let password_hash = hash_password(&req.new_password)?;

    // Clone user_id before consuming user
    let user_id = user.id;

    // Update user password
    let mut user_active: users::ActiveModel = user.into();
    user_active.password_hash = Set(password_hash);
    user_active.update(&state.db).await?;

    // Invalidate all sessions for this user
    sessions::Entity::delete_many()
    .filter(sessions::Column::UserId.eq(user_id))
    .exec(&state.db)
    .await?;

    Ok((StatusCode::OK, Json(ResetPasswordResponse {
        message: "Password reset successfully".to_string(),
    })))
}
