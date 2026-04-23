use serde::{Deserialize, Serialize};
use validator::Validate;

// ============== Rating Criteria Models ==============

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateCriteriaRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub description: Option<String>,
    #[validate(range(min = 0.0, max = 1.0))]
    pub weight: f32,
    #[validate(range(min = 1, max = 100))]
    pub max_score: i32,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateCriteriaRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    pub description: Option<String>,
    #[validate(range(min = 0.0, max = 1.0))]
    pub weight: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReorderCriteriaRequest {
    pub criteria_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CriteriaResponse {
    pub id: String,
    pub hackathon_id: String,
    pub name: String,
    pub description: Option<String>,
    pub weight: f32,
    pub max_score: i32,
    pub sort_order: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CriteriaListResponse {
    pub criteria: Vec<CriteriaResponse>,
}

// ============== Rating Score Input/Detail Models ==============

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ScoreInput {
    #[validate(length(min = 1))]
    pub criteria_id: String,
    #[validate(range(min = 0))]
    pub score: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreDetail {
    pub criteria_id: String,
    pub criteria_name: String,
    pub score: i32,
    pub max_score: i32,
    pub weight: f32,
    pub weighted_score: f32,
}

// ============== Submission Rating Models ==============

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateRatingRequest {
    #[validate(length(min = 1))]
    pub submission_id: String,
    #[validate(length(min = 1))]
    pub scores: Vec<ScoreInput>,
    pub feedback: Option<String>,
    #[serde(default = "default_is_final")]
    pub is_final: bool,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateRatingRequest {
    #[validate(length(min = 1))]
    pub scores: Vec<ScoreInput>,
    pub feedback: Option<String>,
    pub is_final: Option<bool>,
}

fn default_is_final() -> bool {
    true
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RatingResponse {
    pub id: String,
    pub submission_id: String,
    pub organizer_id: String,
    pub organizer_name: String,
    pub total_score: f32,
    pub feedback: Option<String>,
    pub is_final: bool,
    pub scores: Vec<ScoreDetail>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RatingListResponse {
    pub ratings: Vec<RatingResponse>,
}

// ============== Submission with Rating Models ==============

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmissionWithRating {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub repo_url: Option<String>,
    pub demo_url: Option<String>,
    pub status: String,
    pub submitted_at: Option<String>,
    pub team: TeamBrief,
    pub rating: Option<RatingResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamBrief {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmissionsWithRatingsResponse {
    pub submissions: Vec<SubmissionWithRating>,
}

// ============== Public Rating Models ==============

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicRatingEntry {
    pub rank: usize,
    pub team_id: String,
    pub team_name: String,
    pub submission_id: String,
    pub submission_title: String,
    pub total_score: f32,
    pub is_final: bool,
    pub feedback: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicRatingsResponse {
    pub hackathon_id: String,
    pub hackathon_title: String,
    pub ratings: Vec<PublicRatingEntry>,
}
