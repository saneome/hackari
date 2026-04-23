use axum::{
    Json,
    Router,
    extract::{Extension, Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
};
use chrono::Utc;
use sea_orm::{
  ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set, QuerySelect,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    middleware::auth::auth_middleware,
    models::team::*,
    services::{
        auth::Claims,
        state::SharedState,
        websocket::{WsMessage, broadcast_message},
    },
    utils::error::AppError,
};
use entity::{hackathons, organizers, skills, submissions, team_members, teams, tracks, user_skills, users};

use axum::extract::Query;
use std::collections::HashMap;

pub fn routes() -> Router<SharedState> {
  Router::new()
    // Public routes - no auth required
    .route("/ratings/competencies", get(get_competency_ratings))
    .route("/ratings/categories", get(get_rating_categories))
    // Protected routes - auth required
    .route("/", post(create_team))
    .route("/:id", get(get_team).put(update_team).delete(delete_team))
    .route("/:id/join", post(join_team))
    .route("/:id/leave", post(leave_team))
    .route("/:id/submission", get(get_submission).post(create_submission).put(update_submission))
    .layer(middleware::from_fn(auth_middleware))
}

async fn create_team(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateTeamRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let hackathon_id = Uuid::parse_str(&req.hackathon_id)
        .map_err(|_| AppError::BadRequest("Invalid hackathon ID".to_string()))?;

    let hackathon = hackathons::Entity::find_by_id(hackathon_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Hackathon not found".to_string()))?;

    if Utc::now() > hackathon.registration_end {
        return Err(AppError::BadRequest("Registration is closed".to_string()));
    }

    let existing_membership = team_members::Entity::find()
        .inner_join(teams::Entity)
        .filter(teams::Column::HackathonId.eq(hackathon_id))
        .filter(team_members::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    if existing_membership.is_some() {
        return Err(AppError::Conflict("Already in a team for this hackathon".to_string()));
    }

    // Check if user is the organizer of this hackathon
    let organizer = organizers::Entity::find()
        .filter(organizers::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    if let Some(org) = &organizer {
        if org.id == hackathon.organizer_id {
            return Err(AppError::Forbidden("Организатор не может участвовать в своем хакатоне".to_string()));
        }
    }

    let track_id = req.track_id.map(|id| Uuid::parse_str(&id).ok()).flatten();

    let team = teams::ActiveModel {
        hackathon_id: Set(hackathon_id),
        track_id: Set(track_id),
        name: Set(req.name),
        description: Set(req.description),
        status: Set("forming".to_string()),
        ..Default::default()
    };

    let team = team.insert(&state.db).await?;

    let member = team_members::ActiveModel {
        id: Set(Uuid::new_v4()),
        team_id: Set(team.id),
        user_id: Set(claims.sub),
        role: Set("leader".to_string()),
        joined_at: Set(chrono::Utc::now().fixed_offset()),
    };
    member.insert(&state.db).await?;

    let response = build_team_response(&state, team.id).await?;

    Ok((StatusCode::CREATED, Json(response)))
}

async fn get_team(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<TeamResponse>, AppError> {
    let response = build_team_response(&state, id).await?;
    Ok(Json(response))
}

async fn update_team(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTeamRequest>,
) -> Result<Json<TeamResponse>, AppError> {
    let team = teams::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Team not found".to_string()))?;

    let membership = team_members::Entity::find()
        .filter(team_members::Column::TeamId.eq(id))
        .filter(team_members::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    if membership.is_none() || membership.as_ref().unwrap().role != "leader" {
        return Err(AppError::Forbidden("Only team leader can update".to_string()));
    }

    let mut team_active: teams::ActiveModel = team.into();

    if let Some(name) = req.name {
        team_active.name = Set(name);
    }
    if let Some(description) = req.description {
        team_active.description = Set(Some(description));
    }
    if let Some(track_id) = req.track_id {
        if let Ok(id) = Uuid::parse_str(&track_id) {
            team_active.track_id = Set(Some(id));
        }
    }
    if let Some(repo_url) = req.repo_url {
        team_active.repo_url = Set(Some(repo_url));
    }
    if let Some(demo_url) = req.demo_url {
        team_active.demo_url = Set(Some(demo_url));
    }
    if let Some(presentation_url) = req.presentation_url {
        team_active.presentation_url = Set(Some(presentation_url));
    }

    let _team = team_active.update(&state.db).await?;

    let response = build_team_response(&state, id).await?;
    Ok(Json(response))
}

async fn delete_team(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let _team = teams::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Team not found".to_string()))?;

    let membership = team_members::Entity::find()
        .filter(team_members::Column::TeamId.eq(id))
        .filter(team_members::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    if membership.is_none() || membership.as_ref().unwrap().role != "leader" {
        return Err(AppError::Forbidden("Only team leader can delete".to_string()));
    }

    teams::Entity::delete_by_id(id).exec(&state.db).await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn join_team(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(_req): Json<JoinTeamRequest>,
) -> Result<Json<TeamResponse>, AppError> {
    let team = teams::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Team not found".to_string()))?;

    let hackathon = hackathons::Entity::find_by_id(team.hackathon_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Hackathon not found".to_string()))?;

    if Utc::now() > hackathon.event_end {
        return Err(AppError::BadRequest("Hackathon has ended".to_string()));
    }

    let existing_membership = team_members::Entity::find()
        .inner_join(teams::Entity)
        .filter(teams::Column::HackathonId.eq(team.hackathon_id))
        .filter(team_members::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    if existing_membership.is_some() {
        return Err(AppError::Conflict("Already in a team for this hackathon".to_string()));
    }

    // Check if user is the organizer of this hackathon
    let organizer = organizers::Entity::find()
        .filter(organizers::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    if let Some(org) = &organizer {
        if org.id == hackathon.organizer_id {
            return Err(AppError::Forbidden("Организатор не может участвовать в своем хакатоне".to_string()));
        }
    }

    let member_count = team_members::Entity::find()
        .filter(team_members::Column::TeamId.eq(id))
        .count(&state.db)
        .await?;

    if member_count >= 5 {
        return Err(AppError::Conflict("Team is full".to_string()));
    }

    let member = team_members::ActiveModel {
        team_id: Set(id),
        user_id: Set(claims.sub),
        role: Set("member".to_string()),
        ..Default::default()
    };
    member.insert(&state.db).await?;

    let user = users::Entity::find_by_id(claims.sub)
        .one(&state.db)
        .await?;

    if let Some(user) = user {
        broadcast_message(
            &state,
            WsMessage::TeamJoin {
                team_id: id.to_string(),
                user_name: user.name,
            },
        );
    }

    let response = build_team_response(&state, id).await?;
    Ok(Json(response))
}

async fn leave_team(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<LeaveTeamResponse>, AppError> {
    let membership = team_members::Entity::find()
        .filter(team_members::Column::TeamId.eq(id))
        .filter(team_members::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Not a member of this team".to_string()))?;

    let is_leader = membership.role == "leader";
    let member_count = team_members::Entity::find()
        .filter(team_members::Column::TeamId.eq(id))
        .count(&state.db)
        .await?;

    team_members::Entity::delete_by_id(membership.id)
        .exec(&state.db)
        .await?;

    let user = users::Entity::find_by_id(claims.sub)
        .one(&state.db)
        .await?;

    if let Some(user) = user {
        broadcast_message(
            &state,
            WsMessage::TeamLeave {
                team_id: id.to_string(),
                user_name: user.name,
            },
        );
    }

    let mut response = LeaveTeamResponse {
        success: true,
        new_leader: None,
        team_disbanded: false,
    };

    if is_leader && member_count > 1 {
        let new_leader = team_members::Entity::find()
            .filter(team_members::Column::TeamId.eq(id))
            .one(&state.db)
            .await?;

        if let Some(new_leader) = new_leader {
            let mut new_leader_active: team_members::ActiveModel = new_leader.into();
            new_leader_active.role = Set("leader".to_string());
            let leader = new_leader_active.update(&state.db).await?;
            response.new_leader = Some(leader.user_id.to_string());
        }
    } else if member_count == 1 {
        teams::Entity::delete_by_id(id).exec(&state.db).await?;
        response.team_disbanded = true;
    }

    Ok(Json(response))
}

async fn get_submission(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Option<SubmissionResponse>>, AppError> {
    let submission = submissions::Entity::find()
        .filter(submissions::Column::TeamId.eq(id))
        .one(&state.db)
        .await?;

    let response = submission.map(|s| SubmissionResponse {
        id: s.id.to_string(),
        title: s.title,
        description: s.description,
        repo_url: s.repo_url,
        demo_url: s.demo_url,
        status: s.status,
        submitted_at: s.submitted_at.map(|d| d.to_string()),
    });

    Ok(Json(response))
}

async fn create_submission(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(req): Json<CreateSubmissionRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let membership = team_members::Entity::find()
        .filter(team_members::Column::TeamId.eq(id))
        .filter(team_members::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    if membership.is_none() {
        return Err(AppError::Forbidden("Not a team member".to_string()));
    }

    let existing = submissions::Entity::find()
        .filter(submissions::Column::TeamId.eq(id))
        .one(&state.db)
        .await?;

    if existing.is_some() {
        return Err(AppError::Conflict("Submission already exists".to_string()));
    }

    let submission = submissions::ActiveModel {
        team_id: Set(id),
        title: Set(req.title),
        description: Set(req.description),
        repo_url: Set(req.repo_url),
        demo_url: Set(req.demo_url),
        status: Set("draft".to_string()),
        ..Default::default()
    };

    let submission = submission.insert(&state.db).await?;

    let response = SubmissionResponse {
        id: submission.id.to_string(),
        title: submission.title,
        description: submission.description,
        repo_url: submission.repo_url,
        demo_url: submission.demo_url,
        status: submission.status,
        submitted_at: submission.submitted_at.map(|d| d.to_string()),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

async fn update_submission(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateSubmissionRequest>,
) -> Result<Json<SubmissionResponse>, AppError> {
    let membership = team_members::Entity::find()
        .filter(team_members::Column::TeamId.eq(id))
        .filter(team_members::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    if membership.is_none() {
        return Err(AppError::Forbidden("Not a team member".to_string()));
    }

    let submission = submissions::Entity::find()
        .filter(submissions::Column::TeamId.eq(id))
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Submission not found".to_string()))?;

    let mut submission_active: submissions::ActiveModel = submission.into();

    if let Some(title) = req.title {
        submission_active.title = Set(title);
    }
    if let Some(description) = req.description {
        submission_active.description = Set(Some(description));
    }
    if let Some(repo_url) = req.repo_url {
        submission_active.repo_url = Set(Some(repo_url));
    }
    if let Some(demo_url) = req.demo_url {
        submission_active.demo_url = Set(Some(demo_url));
    }

    let submission = submission_active.update(&state.db).await?;

    let response = SubmissionResponse {
        id: submission.id.to_string(),
        title: submission.title,
        description: submission.description,
        repo_url: submission.repo_url,
        demo_url: submission.demo_url,
        status: submission.status,
        submitted_at: submission.submitted_at.map(|d| d.to_string()),
    };

    Ok(Json(response))
}

// Get competency ratings for all teams
async fn get_competency_ratings(
  State(state): State<SharedState>,
  Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<TeamCompetencyRating>>, AppError> {
  // Parse category filter from query params
  let category_filter: Option<String> = params.get("category").cloned();
  
  // Get all teams
  let all_teams = teams::Entity::find()
    .all(&state.db)
    .await?;

    let mut ratings: Vec<TeamCompetencyRating> = Vec::new();

    for team in all_teams {
        // Load hackathon for this team
        let hackathon = hackathons::Entity::find_by_id(team.hackathon_id)
            .one(&state.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Hackathon not found".to_string()))?;
        // Get team members with their user info and skills
        let members = team_members::Entity::find()
            .filter(team_members::Column::TeamId.eq(team.id))
            .all(&state.db)
            .await?;

        let member_count = members.len() as i64;

        // Collect all skills from team members
        let mut team_skills: Vec<(skills::Model, i32)> = Vec::new();
        let mut category_stats: std::collections::HashMap<String, (usize, i32)> = std::collections::HashMap::new();
        let mut unique_skill_names: std::collections::HashSet<String> = std::collections::HashSet::new();

        for member in &members {
            // Get user's skills
            let user_skills_list = user_skills::Entity::find()
                .filter(user_skills::Column::UserId.eq(member.user_id))
                .all(&state.db)
                .await?;

            for user_skill in user_skills_list {
                // Get the skill info separately
                let skill = skills::Entity::find_by_id(user_skill.skill_id)
                    .one(&state.db)
                    .await?
                    .ok_or_else(|| AppError::NotFound("Skill not found".to_string()))?;
                team_skills.push((skill.clone(), user_skill.level));
                unique_skill_names.insert(skill.name.clone());

                let entry = category_stats.entry(skill.category.clone()).or_insert((0, 0));
                entry.0 += 1;
                entry.1 += user_skill.level;
            }
        }

        // Calculate totals
        let total_skill_score: i32 = team_skills.iter().map(|(_, level)| *level).sum();
        let skills_count = unique_skill_names.len();
        let avg_skill_level = if team_skills.is_empty() {
            0.0
        } else {
            total_skill_score as f32 / team_skills.len() as f32
        };

        // Get top 5 skills (by level, then unique)
        let mut top_skills_map: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
        for (skill, level) in &team_skills {
            let entry = top_skills_map.entry(skill.name.clone()).or_insert(0);
            if *level > *entry {
                *entry = *level;
            }
        }

        let mut top_skills_vec: Vec<(String, i32)> = top_skills_map.into_iter().collect();
        top_skills_vec.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

        let top_skills: Vec<TeamSkillInfo> = top_skills_vec
            .into_iter()
            .take(5)
            .map(|(name, level)| TeamSkillInfo { name, level })
            .collect();

        // Build categories with percentages
        let categories: Vec<CategoryCompetency> = category_stats
            .iter()
            .map(|(name, (count, total_level))| {
                let avg_level = *total_level as f32 / *count as f32;
                let percentage = if team_skills.is_empty() {
                    0.0
                } else {
                    (*count as f32 / team_skills.len() as f32) * 100.0
                };
                CategoryCompetency {
                    name: name.clone(),
                    count: *count,
                    avg_level,
                    percentage,
                }
            })
            .collect();

  // Skip teams that don't have the filtered category
  if let Some(ref category) = category_filter {
    if !category_stats.contains_key(category) {
      continue;
    }
  }

  ratings.push(TeamCompetencyRating {
    team_id: team.id.to_string(),
    team_name: team.name.clone(),
    hackathon_id: hackathon.id.to_string(),
    hackathon_name: hackathon.title.clone(),
    member_count,
    total_skill_score,
    skills_count,
    avg_skill_level,
    top_skills,
    categories,
    rank: 0, // Will be calculated after sorting
  });
  }

  // Sort by total skill score desc, then by avg skill level desc
  ratings.sort_by(|a, b| {
    b.total_skill_score
      .cmp(&a.total_skill_score)
      .then_with(|| b.avg_skill_level.partial_cmp(&a.avg_skill_level).unwrap_or(std::cmp::Ordering::Equal))
  });

    // Assign ranks
    for (i, rating) in ratings.iter_mut().enumerate() {
        rating.rank = i + 1;
    }

    Ok(Json(ratings))
}

// Get all unique skill categories for filtering
async fn get_rating_categories(
  State(state): State<SharedState>,
) -> Result<Json<Vec<String>>, AppError> {
  let categories = skills::Entity::find()
    .select_only()
    .column(skills::Column::Category)
    .distinct()
    .into_tuple::<String>()
    .all(&state.db)
    .await?;

  Ok(Json(categories))
}

async fn build_team_response(
    state: &SharedState,
    id: Uuid,
) -> Result<TeamResponse, AppError> {
    let team = teams::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Team not found".to_string()))?;

    let track = if let Some(track_id) = team.track_id {
        tracks::Entity::find_by_id(track_id)
            .one(&state.db)
            .await?
            .map(|t| TrackInfo {
                id: t.id.to_string(),
                name: t.name,
            })
    } else {
        None
    };

    let members = team_members::Entity::find()
        .filter(team_members::Column::TeamId.eq(id))
        .all(&state.db)
        .await?;

    let mut member_responses = Vec::new();
    for member in members {
        let user = users::Entity::find_by_id(member.user_id)
            .one(&state.db)
            .await?;

        if let Some(user) = user {
            member_responses.push(TeamMemberResponse {
                id: member.id.to_string(),
                user_id: member.user_id.to_string(),
                name: user.name,
                avatar_url: user.avatar_url,
                role: member.role,
                joined_at: member.joined_at.to_string(),
            });
        }
    }

    let response = TeamResponse {
        id: team.id.to_string(),
        name: team.name,
        description: team.description,
        hackathon_id: team.hackathon_id.to_string(),
        track,
        repo_url: team.repo_url,
        demo_url: team.demo_url,
        presentation_url: team.presentation_url,
        status: team.status,
        members: member_responses,
        created_at: team.created_at.to_string(),
    };

    Ok(response)
}
