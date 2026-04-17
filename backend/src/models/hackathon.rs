use chrono::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct HackathonResponse {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub banner_url: Option<String>,
    pub location_type: String,
    pub city: Option<String>,
    pub venue: Option<String>,
    pub registration_start: String,
    pub registration_end: String,
    pub event_start: String,
    pub event_end: String,
    pub max_participants: Option<i32>,
    pub organizer: Option<OrganizerResponse>,
    pub is_published: bool,
    pub tracks: Vec<TrackResponse>,
    pub deadlines: Vec<DeadlineResponse>,
    pub participant_count: i64,
    pub team_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizerResponse {
    pub id: String,
    pub name: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub prize_description: Option<String>,
    pub max_teams: Option<i32>,
    pub team_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadlineResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub deadline_at: String,
    pub is_milestone: bool,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateHackathonRequest {
    #[validate(length(min = 3, max = 255))]
    pub title: String,
    pub description: Option<String>,
    #[validate(regex(path = "crate::utils::validators::LOCATION_TYPE_REGEX"))]
    pub location_type: String,
    pub city: Option<String>,
    pub venue: Option<String>,
    pub registration_start: DateTime<chrono::Utc>,
    pub registration_end: DateTime<chrono::Utc>,
    pub event_start: DateTime<chrono::Utc>,
    pub event_end: DateTime<chrono::Utc>,
    pub max_participants: Option<i32>,
    pub tracks: Vec<CreateTrackRequest>,
    pub deadlines: Vec<CreateDeadlineRequest>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateTrackRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: Option<String>,
    pub prize_description: Option<String>,
    pub max_teams: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeadlineRequest {
    pub name: String,
    pub description: Option<String>,
    pub deadline_at: DateTime<chrono::Utc>,
    pub is_milestone: bool,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateHackathonRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub location_type: Option<String>,
    pub city: Option<String>,
    pub venue: Option<String>,
    pub registration_start: Option<DateTime<chrono::Utc>>,
    pub registration_end: Option<DateTime<chrono::Utc>>,
    pub event_start: Option<DateTime<chrono::Utc>>,
    pub event_end: Option<DateTime<chrono::Utc>>,
    pub max_participants: Option<i32>,
    pub is_published: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HackathonListResponse {
    pub hackathons: Vec<HackathonSummary>,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HackathonSummary {
    pub id: String,
    pub title: String,
    pub banner_url: Option<String>,
    pub location_type: String,
    pub registration_start: String,
    pub registration_end: String,
    pub event_start: String,
    pub event_end: String,
    pub participant_count: i64,
    pub team_count: i64,
}
