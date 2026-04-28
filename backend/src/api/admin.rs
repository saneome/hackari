use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    middleware::auth::require_staff_middleware,
    models::admin::*,
    models::organizer::RejectOrganizerRequest,
    services::{auth::Claims, email::EmailService, state::SharedState},
    utils::error::AppError,
};
use entity::{hackathons, organizers, reports, users};

pub fn routes() -> Router<SharedState> {
    Router::new()
        // Dashboard
        .route("/dashboard", get(get_dashboard_stats))
        // Hackathon moderation
        .route("/hackathons/pending", get(list_pending_hackathons))
        .route("/hackathons/:id/approve", post(approve_hackathon))
        .route("/hackathons/:id/reject", post(reject_hackathon))
        // Organizer verification
        .route("/organizers/unverified", get(list_unverified_organizers))
        .route("/organizers/:id/verify", post(verify_organizer))
        .route("/organizers/:id/reject", post(reject_organizer))
        // Reports
        .route("/reports", get(list_reports))
        .route("/reports/:id", get(get_report_detail))
        .route("/reports/:id/resolve", post(resolve_report))
        .route("/reports/:id/close", post(close_report))
        // User management
        .route("/users", get(list_users))
        .route("/users/:id/toggle-staff", post(toggle_user_staff))
        .layer(middleware::from_fn(require_staff_middleware))
}

// Dashboard Stats
async fn get_dashboard_stats(
    State(state): State<SharedState>,
) -> Result<Json<DashboardStats>, AppError> {
    let pending_hackathons_count = hackathons::Entity::find()
        .filter(hackathons::Column::Status.eq("pending"))
        .count(&state.db)
        .await?;

    let unverified_organizers_count = organizers::Entity::find()
        .filter(organizers::Column::IsVerified.eq(false))
        .count(&state.db)
        .await?;

    let open_reports_count = reports::Entity::find()
        .filter(reports::Column::Status.eq("open"))
        .count(&state.db)
        .await?;

    Ok(Json(DashboardStats {
        pending_hackathons_count: pending_hackathons_count as i64,
        unverified_organizers_count: unverified_organizers_count as i64,
        open_reports_count: open_reports_count as i64,
    }))
}

// List pending hackathons
async fn list_pending_hackathons(
    State(state): State<SharedState>,
) -> Result<Json<PaginatedPendingHackathons>, AppError> {
    let hackathons = hackathons::Entity::find()
        .filter(hackathons::Column::Status.eq("pending"))
        .order_by_desc(hackathons::Column::CreatedAt)
        .all(&state.db)
        .await?;

    let total = hackathons.len() as i64;

    let mut pending_hackathons = Vec::new();

    for hackathon in hackathons {
        let organizer = organizers::Entity::find_by_id(hackathon.organizer_id)
            .one(&state.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Organizer not found".to_string()))?;

        pending_hackathons.push(PendingHackathon {
            id: hackathon.id.to_string(),
            title: hackathon.title,
            organizer: OrganizerInfo {
                id: organizer.id.to_string(),
                name: organizer.name,
            },
            created_at: hackathon.created_at.to_rfc3339(),
            description: hackathon.description,
        });
    }

    Ok(Json(PaginatedPendingHackathons {
        hackathons: pending_hackathons,
        total,
    }))
}

// Approve hackathon
async fn approve_hackathon(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    let hackathon = hackathons::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Hackathon not found".to_string()))?;

    if hackathon.status != "pending" {
        return Err(AppError::BadRequest("Hackathon is not pending".to_string()));
    }

    let mut hackathon_active: hackathons::ActiveModel = hackathon.clone().into();
    hackathon_active.status = Set("approved".to_string());
    hackathon_active.is_published = Set(true);
    hackathon_active.update(&state.db).await?;

    // Send email to organizer
    let organizer = organizers::Entity::find_by_id(hackathon.organizer_id)
        .one(&state.db)
        .await?;

    if let Some(org) = organizer {
        let user = users::Entity::find_by_id(org.user_id)
            .one(&state.db)
            .await?;

        if let Some(u) = user {
            let _ = state
                .email_service
                .send_hackathon_approved(&u.email, &hackathon.title, &hackathon.id.to_string())
                .await;
        }
    }

    Ok(StatusCode::OK)
}

// Reject hackathon
async fn reject_hackathon(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
    Json(req): Json<RejectHackathonRequest>,
) -> Result<impl IntoResponse, AppError> {
    let hackathon = hackathons::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Hackathon not found".to_string()))?;

    if hackathon.status != "pending" {
        return Err(AppError::BadRequest("Hackathon is not pending".to_string()));
    }

    let mut hackathon_active: hackathons::ActiveModel = hackathon.clone().into();
    hackathon_active.status = Set("rejected".to_string());
    hackathon_active.update(&state.db).await?;

    // Send email to organizer
    let organizer = organizers::Entity::find_by_id(hackathon.organizer_id)
        .one(&state.db)
        .await?;

    if let Some(org) = organizer {
        let user = users::Entity::find_by_id(org.user_id)
            .one(&state.db)
            .await?;

        if let Some(u) = user {
            let _ = state
                .email_service
                .send_hackathon_rejected(&u.email, &hackathon.title, req.reason.as_deref())
                .await;
        }
    }

    Ok(StatusCode::OK)
}

// List unverified organizers
async fn list_unverified_organizers(
    State(state): State<SharedState>,
) -> Result<Json<PaginatedUnverifiedOrganizers>, AppError> {
    let organizers = organizers::Entity::find()
        .filter(organizers::Column::IsVerified.eq(false))
        .order_by_desc(organizers::Column::CreatedAt)
        .all(&state.db)
        .await?;

    let total = organizers.len() as i64;

    let mut unverified_organizers = Vec::new();

    for organizer in organizers {
        let user = users::Entity::find_by_id(organizer.user_id)
            .one(&state.db)
            .await?;

        if let Some(u) = user {
            unverified_organizers.push(UnverifiedOrganizer {
                id: organizer.id.to_string(),
                name: organizer.name,
                user_id: u.id.to_string(),
                user_name: u.name,
                email: u.email,
                description: organizer.description,
                rejection_reason: organizer.rejection_reason,
                created_at: organizer.created_at.to_rfc3339(),
            });
        }
    }

    Ok(Json(PaginatedUnverifiedOrganizers {
        organizers: unverified_organizers,
        total,
    }))
}

// Verify organizer
async fn verify_organizer(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let organizer = organizers::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Organizer not found".to_string()))?;

    if organizer.is_verified {
        return Err(AppError::BadRequest("Organizer is already verified".to_string()));
    }

    let mut organizer_active: organizers::ActiveModel = organizer.clone().into();
    organizer_active.is_verified = Set(true);
    organizer_active.update(&state.db).await?;

    // Send email to organizer
    let user = users::Entity::find_by_id(organizer.user_id)
        .one(&state.db)
        .await?;

    if let Some(u) = user {
        let _ = state
            .email_service
            .send_organizer_verified(&u.email, &organizer.name)
            .await;
    }

    Ok(StatusCode::OK)
}

// Reject organizer verification
async fn reject_organizer(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
    Json(req): Json<RejectOrganizerRequest>,
) -> Result<impl IntoResponse, AppError> {
    let organizer = organizers::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Organizer not found".to_string()))?;

    if organizer.is_verified {
        return Err(AppError::BadRequest("Organizer is already verified".to_string()));
    }

    let mut organizer_active: organizers::ActiveModel = organizer.clone().into();
    organizer_active.rejection_reason = Set(Some(req.reason.clone()));
    organizer_active.update(&state.db).await?;

    // Send email to organizer
    let user = users::Entity::find_by_id(organizer.user_id)
        .one(&state.db)
        .await?;

    if let Some(u) = user {
        let _ = state
            .email_service
            .send_organizer_rejected(&u.email, &organizer.name, &req.reason)
            .await;
    }

    Ok(StatusCode::OK)
}

// List reports
async fn list_reports(
    State(state): State<SharedState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<PaginatedReports>, AppError> {
    let mut query = reports::Entity::find();

    if let Some(status) = params.get("status") {
        query = query.filter(reports::Column::Status.eq(status));
    }

    let reports = query
        .order_by_desc(reports::Column::CreatedAt)
        .all(&state.db)
        .await?;

    let total = reports.len() as i64;

    let mut report_summaries = Vec::new();

    for report in reports {
        let reporter = users::Entity::find_by_id(report.reporter_id)
            .one(&state.db)
            .await?;

        let target_name = get_target_name(&state.db, &report.target_type, report.target_id).await?;

        report_summaries.push(AdminReport {
            id: report.id.to_string(),
            target_type: report.target_type,
            target_name,
            reason: report.reason,
            status: report.status,
            created_at: report.created_at.to_rfc3339(),
            reporter_name: reporter.map(|r| r.name).unwrap_or_default(),
        });
    }

    Ok(Json(PaginatedReports {
        reports: report_summaries,
        total,
    }))
}

// Get report detail
async fn get_report_detail(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ReportDetail>, AppError> {
    let report = reports::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Report not found".to_string()))?;

    let reporter = users::Entity::find_by_id(report.reporter_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Reporter not found".to_string()))?;

    let target_name = get_target_name(&state.db, &report.target_type, report.target_id).await?;

    let resolved_by = if let Some(resolver_id) = report.resolved_by {
        users::Entity::find_by_id(resolver_id)
            .one(&state.db)
            .await?
            .map(|u| ResolverInfo {
                id: u.id.to_string(),
                name: u.name,
            })
    } else {
        None
    };

    Ok(Json(ReportDetail {
        id: report.id.to_string(),
        reporter: ReporterInfo {
            id: reporter.id.to_string(),
            name: reporter.name.clone(),
            email: reporter.email,
        },
        target_type: report.target_type.clone(),
        target_id: report.target_id.to_string(),
        target_name,
        reason: report.reason,
        description: report.description,
        status: report.status,
        created_at: report.created_at.to_rfc3339(),
        resolved_by,
        resolved_at: report.resolved_at.map(|d| d.to_rfc3339()),
        resolution_note: report.resolution_note,
    }))
}

// Resolve report
async fn resolve_report(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<ResolveReportRequest>,
) -> Result<impl IntoResponse, AppError> {
    let report = reports::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Report not found".to_string()))?;

    if report.status != "open" {
        return Err(AppError::BadRequest("Report is not open".to_string()));
    }

    let mut report_active: reports::ActiveModel = report.clone().into();
    report_active.status = Set("resolved".to_string());
    report_active.resolved_by = Set(Some(claims.sub));
    report_active.resolved_at = Set(Some(Utc::now().into()));
    report_active.resolution_note = Set(Some(req.resolution_note));
    report_active.update(&state.db).await?;

    // Send email to reporter
    let reporter = users::Entity::find_by_id(report.reporter_id)
        .one(&state.db)
        .await?;

    if let Some(r) = reporter {
        let _ = state
            .email_service
            .send_report_resolved(&r.email, &report.id.to_string())
            .await;
    }

    Ok(StatusCode::OK)
}

// Close report without resolution
async fn close_report(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    let report = reports::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Report not found".to_string()))?;

    if report.status != "open" {
        return Err(AppError::BadRequest("Report is not open".to_string()));
    }

    let mut report_active: reports::ActiveModel = report.clone().into();
    report_active.status = Set("closed".to_string());
    report_active.resolved_by = Set(Some(claims.sub));
    report_active.resolved_at = Set(Some(Utc::now().into()));
    report_active.update(&state.db).await?;

    Ok(StatusCode::OK)
}

// List users
async fn list_users(
    State(state): State<SharedState>,
) -> Result<Json<PaginatedUsers>, AppError> {
    let users = users::Entity::find()
        .order_by_desc(users::Column::CreatedAt)
        .all(&state.db)
        .await?;

    let total = users.len() as i64;

    let user_infos: Vec<AdminUserInfo> = users
        .into_iter()
        .map(|u| AdminUserInfo {
            id: u.id.to_string(),
            email: u.email,
            name: u.name,
            is_staff: u.is_staff,
            is_superuser: u.is_superuser,
            created_at: u.created_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(PaginatedUsers {
        users: user_infos,
        total,
    }))
}

// Toggle user staff status
async fn toggle_user_staff(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    // Only superuser can toggle staff status
    if !claims.is_superuser {
        return Err(AppError::Forbidden("Only superuser can toggle staff status".to_string()));
    }

    // Cannot modify self
    if id == claims.sub {
        return Err(AppError::BadRequest("Cannot modify your own staff status".to_string()));
    }

    let user = users::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    let new_status = !user.is_staff;
    let mut user_active: users::ActiveModel = user.into();
    user_active.is_staff = Set(new_status);
    user_active.update(&state.db).await?;

    Ok(StatusCode::OK)
}

// Helper function to get target name
async fn get_target_name(
    db: &sea_orm::DatabaseConnection,
    target_type: &str,
    target_id: Uuid,
) -> Result<String, AppError> {
    match target_type {
        "hackathon" => {
            let hackathon = hackathons::Entity::find_by_id(target_id)
                .one(db)
                .await?;
            Ok(hackathon.map(|h| h.title).unwrap_or_else(|| "Unknown".to_string()))
        }
        "organizer" => {
            let organizer = organizers::Entity::find_by_id(target_id)
                .one(db)
                .await?;
            Ok(organizer.map(|o| o.name).unwrap_or_else(|| "Unknown".to_string()))
        }
        "team" => {
            let team = entity::teams::Entity::find_by_id(target_id)
                .one(db)
                .await?;
            Ok(team.map(|t| t.name).unwrap_or_else(|| "Unknown".to_string()))
        }
        "user" => {
            let user = users::Entity::find_by_id(target_id)
                .one(db)
                .await?;
            Ok(user.map(|u| u.name).unwrap_or_else(|| "Unknown".to_string()))
        }
        _ => Ok("Unknown".to_string()),
    }
}
