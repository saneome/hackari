use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    pub pending_hackathons_count: i64,
    pub unverified_organizers_count: i64,
    pub open_reports_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizerInfo {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PendingHackathon {
    pub id: String,
    pub title: String,
    pub organizer: OrganizerInfo,
    pub created_at: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnverifiedOrganizer {
    pub id: String,
    pub name: String,
    pub user_id: String,
    pub user_name: String,
    pub email: String,
    pub description: Option<String>,
    pub rejection_reason: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminReport {
    pub id: String,
    pub target_type: String,
    pub target_name: String,
    pub reason: String,
    pub status: String,
    pub created_at: String,
    pub reporter_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReporterInfo {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResolverInfo {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportDetail {
    pub id: String,
    pub reporter: ReporterInfo,
    pub target_type: String,
    pub target_id: String,
    pub target_name: String,
    pub reason: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: String,
    pub resolved_by: Option<ResolverInfo>,
    pub resolved_at: Option<String>,
    pub resolution_note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminUserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
    pub is_staff: bool,
    pub is_superuser: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RejectHackathonRequest {
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResolveReportRequest {
    pub resolution_note: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedPendingHackathons {
    pub hackathons: Vec<PendingHackathon>,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedUnverifiedOrganizers {
    pub organizers: Vec<UnverifiedOrganizer>,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedReports {
    pub reports: Vec<AdminReport>,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedUsers {
    pub users: Vec<AdminUserInfo>,
    pub total: i64,
}
