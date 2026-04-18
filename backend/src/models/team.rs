use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub hackathon_id: String,
    pub track: Option<TrackInfo>,
    pub repo_url: Option<String>,
    pub demo_url: Option<String>,
    pub presentation_url: Option<String>,
    pub status: String,
    pub members: Vec<TeamMemberResponse>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackInfo {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamMemberResponse {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub role: String,
    pub joined_at: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateTeamRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: String,
    pub description: Option<String>,
    pub track_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateTeamRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub track_id: Option<String>,
    pub repo_url: Option<String>,
    pub demo_url: Option<String>,
    pub presentation_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinTeamRequest {
    pub invite_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaveTeamResponse {
    pub success: bool,
    pub new_leader: Option<String>,
    pub team_disbanded: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamListResponse {
    pub teams: Vec<TeamSummary>,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamSummary {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub hackathon_id: String,
    pub track_name: Option<String>,
    pub status: String,
    pub member_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmissionResponse {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub repo_url: Option<String>,
    pub demo_url: Option<String>,
    pub status: String,
    pub submitted_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateSubmissionRequest {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    pub description: Option<String>,
    pub repo_url: Option<String>,
    pub demo_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateSubmissionRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub repo_url: Option<String>,
    pub demo_url: Option<String>,
}

// Competency Rating models
#[derive(Debug, Serialize, Deserialize)]
pub struct TeamCompetencyRating {
    pub team_id: String,
    pub team_name: String,
    pub hackathon_id: String,
    pub hackathon_name: String,
    pub member_count: i64,
    pub total_skill_score: i32,
    pub skills_count: usize,
    pub avg_skill_level: f32,
    pub top_skills: Vec<TeamSkillInfo>,
    pub categories: Vec<CategoryCompetency>,
    pub rank: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamSkillInfo {
    pub name: String,
    pub level: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryCompetency {
    pub name: String,
    pub count: usize,
    pub avg_level: f32,
    pub percentage: f32,
}
