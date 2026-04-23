use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(length(min = 2, max = 100))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub github_url: Option<String>,
    pub telegram_username: Option<String>,
    pub is_verified: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RequestResetRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct VerifyResetCodeRequest {
    pub email: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ResetPasswordRequest {
    pub token: String,
    #[validate(length(min = 8))]
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResetCodeResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResetPasswordResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationResponse {
    pub message: String,
}

/// Pending registration data stored in Redis until email is verified
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PendingRegistration {
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub created_at: String, // ISO 8601 timestamp
}

impl PendingRegistration {
    pub fn new(email: String, password_hash: String, name: String) -> Self {
        Self {
            email,
            password_hash,
            name,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}
