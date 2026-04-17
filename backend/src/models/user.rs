use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfileResponse {
    pub id: String,
    pub email: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub github_url: Option<String>,
    pub telegram_username: Option<String>,
    pub is_verified: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: Option<String>,
    pub bio: Option<String>,
    pub github_url: Option<String>,
    pub telegram_username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadAvatarResponse {
    pub avatar_url: String,
}
