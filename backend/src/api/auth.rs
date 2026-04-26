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
use uuid::Uuid;

use crate::{
    middleware::auth::optional_auth_middleware,
    models::auth::*,
    services::{
        auth::*,
        state::SharedState,
    },
    utils::error::AppError,
};
use serde_json;
use entity::{sessions, users};

// Redis token expiration in seconds
const PASSWORD_RESET_TTL: u64 = 900; // 15 minutes
const EMAIL_VERIFICATION_TTL: u64 = 86400; // 24 hours

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/refresh", post(refresh_token))
        .route("/me", get(get_current_user).layer(middleware::from_fn(optional_auth_middleware)))
        .route("/password-reset/request", post(request_reset))
        .route("/password-reset/reset", post(reset_password))
        .route("/verify-email/:token", get(verify_email))
        .route("/resend-verification", post(resend_verification))
}

async fn register(
    State(state): State<SharedState>,
    _jar: CookieJar,
    Json(req): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    if !req.terms_accepted || !req.privacy_accepted {
        return Err(AppError::BadRequest(
            "Необходимо принять условия использования и дать согласие на обработку персональных данных".to_string(),
        ));
    }

    // Check if email exists in database
    let existing_user = users::Entity::find()
        .filter(users::Column::Email.eq(&req.email))
        .one(&state.db)
        .await?;

    if let Some(user) = existing_user {
        if user.is_verified {
            // User is verified - cannot re-register
            return Err(AppError::Conflict("Email already registered".to_string()));
        } else {
            // User exists but not verified - delete old record and allow re-registration
            tracing::info!("Deleting unverified user with email: {}", req.email);
            users::Entity::delete_by_id(user.id)
                .exec(&state.db)
                .await?;
        }
    }

    // Check if there's already a pending registration for this email
    let mut redis_conn = state.redis.clone();
    let pending_key = format!("pending_reg:{}", req.email);
    let existing_pending: Option<String> = redis_conn.get(&pending_key).await?;

    if existing_pending.is_some() {
        // Delete old pending registration to allow re-registration
        tracing::info!("Deleting old pending registration for email: {}", req.email);
        let _: () = redis_conn.del(&pending_key).await?;
    }

    let password_hash = hash_password(&req.password)?;

    // Create pending registration in Redis
    let pending_reg = PendingRegistration::new(
        req.email.clone(),
        password_hash,
        req.name.clone(),
        req.terms_accepted,
        req.privacy_accepted,
    );

    let pending_json = serde_json::to_string(&pending_reg)
        .map_err(|e| AppError::Internal(format!("Failed to serialize registration: {}", e)))?;

    // Store pending registration with TTL
    let _: () = redis_conn.set_ex(&pending_key, &pending_json, EMAIL_VERIFICATION_TTL).await?;

    // Generate verification token (links to the pending registration)
    let verification_token = Uuid::new_v4().to_string();
    let verify_token_key = format!("email_verify:{}", verification_token);
    let _: () = redis_conn.set_ex(&verify_token_key, &req.email, EMAIL_VERIFICATION_TTL).await?;

    // Send verification email
    let frontend_url = std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());
    let verify_url = format!("{}/auth/verify-email?token={}", frontend_url, verification_token);
    tracing::debug!("Verification URL: {}", verify_url);

    if let Err(e) = state.email_service.send_email_verification(&req.email, &verification_token).await {
        tracing::error!("Failed to send verification email: {:?}", e);
        // For development: log the verification URL so developer can manually verify
        tracing::info!("DEV: Verification link for {}: {}", req.email, verify_url);
    }

    Ok((StatusCode::CREATED, Json(serde_json::json!({
        "message": "Registration successful. Please check your email to verify your account. The link is valid for 24 hours.",
        "email": req.email,
    }))))
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
        .ok_or_else(|| AppError::Unauthorized("Неверный email или пароль".to_string()))?;

    // Check if user is verified
    if !user.is_verified {
        return Err(AppError::Forbidden("Email не подтверждён. Проверьте почту или запросите новую ссылку".to_string()));
    }

    let valid = verify_password(&req.password, &user.password_hash)?;
    if !valid {
        return Err(AppError::Unauthorized("Неверный email или пароль".to_string()));
    }

    let access_token = generate_access_token(user.id, &user.email, user.is_staff, user.is_superuser)?;
    let refresh_token = generate_refresh_token(user.id, &user.email, user.is_staff, user.is_superuser)?;

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
        terms_accepted_at: user.terms_accepted_at.map(|dt| dt.to_rfc3339()),
        privacy_accepted_at: user.privacy_accepted_at.map(|dt| dt.to_rfc3339()),
        is_staff: user.is_staff,
        is_superuser: user.is_superuser,
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
        .ok_or_else(|| AppError::Unauthorized("No refresh token".to_string()))?;

    let claims = decode_token(&refresh_token)?;

    if claims.token_type != "refresh" {
        return Err(AppError::Unauthorized("Invalid token type".to_string()));
    }

    let user_id = claims.sub;
    let user = users::Entity::find_by_id(user_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::Unauthorized("User not found".to_string()))?;

    // Check if user is still verified
    if !user.is_verified {
        return Err(AppError::Forbidden("Email не подтверждён".to_string()));
    }

    let access_token = generate_access_token(user.id, &user.email, user.is_staff, user.is_superuser)?;
    let refresh_token = generate_refresh_token(user.id, &user.email, user.is_staff, user.is_superuser)?;

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
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        let user_response = UserResponse {
            id: user.id.to_string(),
            email: user.email,
            name: user.name,
            avatar_url: user.avatar_url,
            bio: user.bio,
            github_url: user.github_url,
            telegram_username: user.telegram_username,
            is_verified: user.is_verified,
            terms_accepted_at: user.terms_accepted_at.map(|dt| dt.to_rfc3339()),
            privacy_accepted_at: user.privacy_accepted_at.map(|dt| dt.to_rfc3339()),
            is_staff: user.is_staff,
            is_superuser: user.is_superuser,
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
        return Ok((StatusCode::OK, Json(serde_json::json!({
            "message": "Если email существует, ссылка для сброса пароля отправлена"
        }))));
    };

    // Generate reset token
    let reset_token = Uuid::new_v4().to_string();
    let redis_key = format!("password_reset:{}", reset_token);
    let mut redis_conn = state.redis.clone();
    let _: () = redis_conn.set_ex(&redis_key, user.id.to_string(), PASSWORD_RESET_TTL).await?;

    // Send email with reset link
    let frontend_url = std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());
    let reset_url = format!("{}/auth/reset-password?token={}", frontend_url, reset_token);
    tracing::debug!("Reset URL: {}", reset_url);

    match state.email_service.send_password_reset_link(&req.email, &reset_token).await {
        Ok(_) => {
            Ok((StatusCode::OK, Json(serde_json::json!({
                "message": "Если email существует, ссылка для сброса пароля отправлена"
            }))))
        }
        Err(e) => {
            tracing::error!("Failed to send reset email: {:?}", e);
            // For development: log the reset URL so developer can test manually
            tracing::info!("DEV: Reset link for {}: {}", req.email, reset_url);
            // Still return success to mask email existence
            Ok((StatusCode::OK, Json(serde_json::json!({
                "message": "Если email существует, ссылка для сброса пароля отправлена"
            }))))
        }
    }
}

async fn reset_password(
    State(state): State<SharedState>,
    Json(req): Json<ResetPasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Validate token from Redis
    let redis_key = format!("password_reset:{}", req.token);
    let mut redis_conn = state.redis.clone();
    let user_id_str: Option<String> = redis_conn.get(&redis_key).await?;

    let Some(user_id_str) = user_id_str else {
        return Err(AppError::BadRequest("Ссылка устарела или недействительна".to_string()));
    };

    // Delete token (one-time use)
    let _: () = redis_conn.del(&redis_key).await?;

    let user_id: Uuid = user_id_str.parse()
        .map_err(|_| AppError::Internal("Invalid user ID in token".to_string()))?;

    // Find user
    let user = users::Entity::find_by_id(user_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    // Hash new password
    let password_hash = hash_password(&req.new_password)?;

    // Update user password
    let mut user_active: users::ActiveModel = user.into();
    user_active.password_hash = Set(password_hash);
    user_active.update(&state.db).await?;

    // Invalidate all sessions for this user
    sessions::Entity::delete_many()
        .filter(sessions::Column::UserId.eq(user_id))
        .exec(&state.db)
        .await?;

    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": "Пароль успешно изменён"
    }))))
}

// Email verification endpoint
async fn verify_email(
    State(state): State<SharedState>,
    axum::extract::Path(token): axum::extract::Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let redis_key = format!("email_verify:{}", token);
    let mut redis_conn = state.redis.clone();
    let email: Option<String> = redis_conn.get(&redis_key).await?;

    let Some(email) = email else {
        return Err(AppError::BadRequest("Ссылка устарела или недействительна".to_string()));
    };

    // Get pending registration data
    let pending_key = format!("pending_reg:{}", email);
    let pending_json: Option<String> = redis_conn.get(&pending_key).await?;

    let Some(pending_json) = pending_json else {
        // Token exists but pending registration expired - delete token
        let _: () = redis_conn.del(&redis_key).await?;
        return Err(AppError::BadRequest("Срок подтверждения истёк. Пожалуйста, зарегистрируйтесь заново.".to_string()));
    };

    // Parse pending registration
    let pending_reg: PendingRegistration = serde_json::from_str(&pending_json)
        .map_err(|e| AppError::Internal(format!("Failed to parse registration data: {}", e)))?;

    // Check if user already exists (race condition protection)
    let existing = users::Entity::find()
        .filter(users::Column::Email.eq(&email))
        .one(&state.db)
        .await?;

    if existing.is_some() {
        // Clean up Redis
        let _: () = redis_conn.del(&redis_key).await?;
        let _: () = redis_conn.del(&pending_key).await?;
        return Err(AppError::Conflict("Email already registered".to_string()));
    }

    // Create user in database
    let now = chrono::Utc::now().fixed_offset();
    let user = users::ActiveModel {
        email: Set(pending_reg.email),
        password_hash: Set(pending_reg.password_hash),
        name: Set(pending_reg.name),
        is_verified: Set(true),
        terms_accepted_at: Set(if pending_reg.terms_accepted { Some(now) } else { None }),
        privacy_accepted_at: Set(if pending_reg.privacy_accepted { Some(now) } else { None }),
        ..Default::default()
    };

    let user = user.insert(&state.db).await?;

    // Clean up Redis - delete both token and pending registration
    let _: () = redis_conn.del(&redis_key).await?;
    let _: () = redis_conn.del(&pending_key).await?;

    tracing::info!("User {} verified and created in database", user.id);

    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": "Email успешно подтверждён. Теперь вы можете войти в аккаунт.",
        "user_id": user.id.to_string(),
    }))))
}

// Resend verification email
#[derive(Deserialize, Validate)]
struct ResendVerificationRequest {
    #[validate(email)]
    email: String,
}

async fn resend_verification(
    State(state): State<SharedState>,
    Json(req): Json<ResendVerificationRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let mut redis_conn = state.redis.clone();

    // Check if there's a pending registration in Redis
    let pending_key = format!("pending_reg:{}", req.email);
    let pending_json: Option<String> = redis_conn.get(&pending_key).await?;

    // Also check if user already exists in database
    let existing_user = users::Entity::find()
        .filter(users::Column::Email.eq(&req.email))
        .one(&state.db)
        .await?;

    if existing_user.is_some() {
        // User exists in database
        if existing_user.unwrap().is_verified {
            return Ok((StatusCode::OK, Json(serde_json::json!({
                "message": "Email уже подтверждён"
            }))));
        }
        // This shouldn't happen - verified user in DB but we shouldn't get here
        return Ok((StatusCode::OK, Json(serde_json::json!({
            "message": "Email уже подтверждён"
        }))));
    }

    // Check if there's a pending registration
    if pending_json.is_none() {
        // No pending registration and no user in DB
        return Ok((StatusCode::OK, Json(serde_json::json!({
            "message": "Если email зарегистрирован и не подтверждён, письмо отправлено"
        }))));
    }

    // Generate new verification token
    let verification_token = Uuid::new_v4().to_string();
    let verify_key = format!("email_verify:{}", verification_token);

    // Store new token with TTL
    let _: () = redis_conn.set_ex(&verify_key, &req.email, EMAIL_VERIFICATION_TTL).await?;

    // Send verification email
    match state.email_service.send_email_verification(&req.email, &verification_token).await {
        Ok(_) => {
            Ok((StatusCode::OK, Json(serde_json::json!({
                "message": "Если email зарегистрирован и не подтверждён, письмо отправлено"
            }))))
        }
        Err(e) => {
            tracing::error!("Failed to send verification email: {:?}", e);
            // Clean up the token we just created
            let _: () = redis_conn.del(&verify_key).await?;
            Err(AppError::Internal("Ошибка отправки письма".to_string()))
        }
    }
}
