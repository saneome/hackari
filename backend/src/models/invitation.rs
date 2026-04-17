use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateInvitationRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvitationResponse {
    pub id: String,
    pub team_id: String,
    pub team_name: String,
    pub invited_by: UserInfo,
    pub invited_email: String,
    pub status: String,
    pub expires_at: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptInvitationRequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvitationListResponse {
    pub invitations: Vec<InvitationResponse>,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PendingInvitationResponse {
    pub id: String,
    pub team: TeamInfo,
    pub invited_by: UserInfo,
    pub expires_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamInfo {
    pub id: String,
    pub name: String,
    pub hackathon_name: String,
}
