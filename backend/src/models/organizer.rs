use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateOrganizerRequest {
    #[validate(length(min = 1, max = 255, message = "Название организатора обязательно"))]
    pub name: String,
    #[validate(length(min = 1, max = 20, message = "Тип организатора обязателен"))]
    pub type_: String,
    pub description: Option<String>,
    #[validate(url(message = "Некорректный URL"))]
    pub website_url: Option<String>,
    pub logo_url: Option<String>,
    #[validate(email(message = "Некорректный email"))]
    pub email: String,
    pub social_links: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateOrganizerRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    pub description: Option<String>,
    #[validate(url(message = "Некорректный URL"))]
    pub website_url: Option<String>,
    pub logo_url: Option<String>,
    #[validate(email(message = "Некорректный email"))]
    pub email: Option<String>,
    pub social_links: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct OrganizerResponse {
    pub id: String,
    pub user_id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub description: Option<String>,
    pub website_url: Option<String>,
    pub logo_url: Option<String>,
    pub email: String,
    pub social_links: Option<serde_json::Value>,
    pub is_verified: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct OrganizerPublicResponse {
    pub id: String,
    pub name: String,
    pub type_: String,
    pub description: Option<String>,
    pub website_url: Option<String>,
    pub logo_url: Option<String>,
    pub social_links: Option<serde_json::Value>,
    pub is_verified: bool,
}

#[derive(Debug, Serialize)]
pub struct OrganizerWithHackathonsResponse {
    pub id: String,
    pub name: String,
    pub type_: String,
    pub description: Option<String>,
    pub website_url: Option<String>,
    pub logo_url: Option<String>,
    pub social_links: Option<serde_json::Value>,
    pub is_verified: bool,
    pub hackathons: Vec<HackathonSummary>,
}

#[derive(Debug, Serialize)]
pub struct HackathonSummary {
    pub id: String,
    pub title: String,
    pub banner_url: Option<String>,
    pub event_start: String,
    pub event_end: String,
    pub location_type: String,
    pub is_published: bool,
}
