use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReportRequest {
    pub target_type: String,
    pub target_id: String,
    pub reason: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyReportSummary {
    pub id: String,
    pub target_type: String,
    pub target_name: String,
    pub reason: String,
    pub status: String,
    pub created_at: String,
    pub resolved_at: Option<String>,
}
