use axum::{
    extract::{Extension, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::{
    middleware::auth::auth_middleware,
    models::report::*,
    services::{auth::Claims, state::SharedState},
    utils::error::AppError,
};
use entity::{hackathons, organizers, reports, teams, users};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/", post(create_report))
        .route("/my", get(get_my_reports))
        .layer(middleware::from_fn(auth_middleware))
}

async fn create_report(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateReportRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate target_type
    let valid_types = ["hackathon", "organizer", "team", "user"];
    if !valid_types.contains(&req.target_type.as_str()) {
        return Err(AppError::BadRequest("Invalid target_type".to_string()));
    }

    // Validate target_id is a valid UUID
    let target_id = Uuid::parse_str(&req.target_id)
        .map_err(|_| AppError::BadRequest("Invalid target_id format".to_string()))?;

    // Verify target exists based on type
    let target_name = match req.target_type.as_str() {
        "hackathon" => {
            let hackathon = hackathons::Entity::find_by_id(target_id)
                .one(&state.db)
                .await?
                .ok_or(AppError::NotFound("Target not found".to_string()))?;
            hackathon.title
        }
        "organizer" => {
            let organizer = organizers::Entity::find_by_id(target_id)
                .one(&state.db)
                .await?
                .ok_or(AppError::NotFound("Target not found".to_string()))?;
            organizer.name
        }
        "team" => {
            let team = teams::Entity::find_by_id(target_id)
                .one(&state.db)
                .await?
                .ok_or(AppError::NotFound("Target not found".to_string()))?;
            team.name
        }
        "user" => {
            let user = users::Entity::find_by_id(target_id)
                .one(&state.db)
                .await?
                .ok_or(AppError::NotFound("Target not found".to_string()))?;
            user.name
        }
        _ => unreachable!(),
    };

    // Prevent self-reporting for users
    if req.target_type == "user" && target_id == claims.sub {
        return Err(AppError::BadRequest("Cannot report yourself".to_string()));
    }

    let report = reports::ActiveModel {
        id: Set(Uuid::new_v4()),
        reporter_id: Set(claims.sub),
        target_type: Set(req.target_type),
        target_id: Set(target_id),
        reason: Set(req.reason),
        description: Set(req.description),
        status: Set("open".to_string()),
        created_at: Set(Utc::now().into()),
        resolved_by: Set(None),
        resolved_at: Set(None),
        resolution_note: Set(None),
    };

    report.insert(&state.db).await?;

    Ok(StatusCode::CREATED)
}

async fn get_my_reports(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<MyReportSummary>>, AppError> {
    let reports = reports::Entity::find()
        .filter(reports::Column::ReporterId.eq(claims.sub))
        .all(&state.db)
        .await?;

    let summaries: Vec<MyReportSummary> = reports
        .into_iter()
        .map(|r| {
            let target_name = get_target_name_sync(&state, &r.target_type, r.target_id);
            MyReportSummary {
                id: r.id.to_string(),
                target_type: r.target_type,
                target_name,
                reason: r.reason,
                status: r.status,
                created_at: r.created_at.to_rfc3339(),
                resolved_at: r.resolved_at.map(|d| d.to_rfc3339()),
            }
        })
        .collect();

    Ok(Json(summaries))
}

fn get_target_name_sync(state: &SharedState, target_type: &str, target_id: Uuid) -> String {
    // This is a synchronous placeholder - in production, fetch this separately
    match target_type {
        "hackathon" => "Hackathon".to_string(),
        "organizer" => "Organizer".to_string(),
        "team" => "Team".to_string(),
        "user" => "User".to_string(),
        _ => "Unknown".to_string(),
    }
}
