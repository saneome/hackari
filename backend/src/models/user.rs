use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillResponse {
    pub id: String,
    pub name: String,
    pub category: String,
    pub level: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfileResponse {
    pub id: String,
    pub email: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub github_url: Option<String>,
    pub telegram_username: Option<String>,
    pub is_verified: bool,
    pub organizer_terms_accepted_at: Option<String>,
    pub created_at: String,
    pub skills: Vec<SkillResponse>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddSkillRequest {
    pub skill_id: String,
    pub level: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveSkillRequest {
    pub skill_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AvailableSkillResponse {
    pub id: String,
    pub name: String,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSkillLevelRequest {
    pub level: i32,
}
