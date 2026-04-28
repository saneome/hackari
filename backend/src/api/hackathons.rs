use axum::{
    Json,
    Router,
    extract::{Extension, Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post, put},
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    Set, TransactionTrait,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    middleware::auth::optional_auth_middleware,
    services::auth::Claims,
    models::hackathon::*,
    models::team::TeamSummary,
    services::state::SharedState,
    utils::error::AppError,
};
use entity::{deadlines, hackathons, team_members, teams, tracks, users, organizers, skills, hackathon_skill};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(list_hackathons).post(create_hackathon))
        .route("/my", get(list_my_hackathons))
        .route("/:id", get(get_hackathon).put(update_hackathon).delete(delete_hackathon))
        .route("/:id/cancel", post(cancel_hackathon))
        .route("/:id/participants", get(get_hackathon_participants))
        .route("/:id/teams", get(get_hackathon_teams))
        // Rating criteria endpoints
        .route("/:id/criteria", get(get_hackathon_criteria).post(create_criteria))
        .route("/:id/criteria/:criteria_id", put(update_criteria).delete(delete_criteria))
        .route("/:id/criteria/reorder", post(reorder_criteria))
        // Submission rating endpoints
        .route("/:id/submissions/with-ratings", get(get_submissions_with_ratings))
        .route("/:id/ratings", post(create_rating))
        .route("/:id/ratings/:rating_id", put(update_rating).delete(delete_rating))
        .route("/:id/ratings/public", get(get_public_ratings))
        .layer(middleware::from_fn(optional_auth_middleware))
}

async fn list_hackathons(
    State(state): State<SharedState>,
) -> Result<Json<HackathonListResponse>, AppError> {
    // Only show approved and published hackathons to the public
    let hackathons = hackathons::Entity::find()
        .filter(hackathons::Column::IsPublished.eq(true))
        .filter(hackathons::Column::Status.eq("approved"))
        .filter(hackathons::Column::EventEnd.gte(Utc::now()))
        .all(&state.db)
        .await?;

    let summaries: Vec<HackathonSummary> = hackathons
        .into_iter()
        .map(|h| HackathonSummary {
            id: h.id.to_string(),
            title: h.title,
            banner_url: h.banner_url,
            location_type: h.location_type,
            registration_start: h.registration_start.to_string(),
            registration_end: h.registration_end.to_string(),
            event_start: h.event_start.to_string(),
            event_end: h.event_end.to_string(),
            participant_count: 0,
            team_count: 0,
        })
        .collect();

    let total = summaries.len() as i64;

    Ok(Json(HackathonListResponse {
        hackathons: summaries,
        total,
    }))
}

async fn create_hackathon(
    State(state): State<SharedState>,
    claims: Option<Extension<Claims>>,
    Json(req): Json<CreateHackathonRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let claims = claims
        .map(|Extension(claims)| claims)
        .ok_or(AppError::Unauthorized("Требуется авторизация".to_string()))?;

    // Check if user has an organizer profile
    let organizer = organizers::Entity::find()
        .filter(organizers::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    let organizer = match organizer {
        Some(org) => org,
        None => {
            return Err(AppError::Forbidden(
                "Ваш профиль организатора не верифицирован. Создание хакатонов доступно только после прохождения верификации.".to_string(),
            ));
        }
    };

    if !organizer.is_verified {
        return Err(AppError::Forbidden(
            "Ваш профиль организатора не верифицирован. Создание хакатонов доступно только после прохождения верификации.".to_string(),
        ));
    }

    let organizer_id = organizer.id;

    // Create hackathon in transaction
    let tx = state.db.begin().await?;

    let hackathon = hackathons::ActiveModel {
        id: Set(Uuid::new_v4()),
        title: Set(req.title),
        description: Set(req.description),
        banner_url: Set(None),
        location_type: Set(req.location_type),
        city: Set(req.city),
        venue: Set(req.venue),
        registration_start: Set(req.registration_start.into()),
        registration_end: Set(req.registration_end.into()),
        event_start: Set(req.event_start.into()),
        event_end: Set(req.event_end.into()),
        max_participants: Set(req.max_participants),
        organizer_id: Set(organizer_id),
        is_published: Set(false), // Not published until approved
        status: Set("pending".to_string()), // Requires moderation approval
        contact_email: Set(req.contact_email),
        website_url: Set(req.website_url),
        social_links: Set(req.social_links),
        prize_pool: Set(req.prize_pool),
        prize_currency: Set(req.prize_currency),
        prize_description: Set(req.prize_description),
        requirements: Set(req.requirements),
        team_size_min: Set(req.team_size_min),
        team_size_max: Set(req.team_size_max),
        age_restriction: Set(req.age_restriction),
        created_at: Set(Utc::now().into()),
        updated_at: Set(Utc::now().into()),
    };

    let hackathon = hackathon.insert(&tx).await?;

    // Create tracks
    for track_req in req.tracks {
        let track = tracks::ActiveModel {
            id: Set(Uuid::new_v4()),
            hackathon_id: Set(hackathon.id),
            name: Set(track_req.name),
            description: Set(track_req.description),
            prize_description: Set(track_req.prize_description),
            max_teams: Set(track_req.max_teams),
            ..Default::default()
        };
        track.insert(&tx).await?;
    }

    // Create deadlines
    for deadline_req in req.deadlines {
        let deadline = deadlines::ActiveModel {
            id: Set(Uuid::new_v4()),
            hackathon_id: Set(hackathon.id),
            name: Set(deadline_req.name),
            description: Set(deadline_req.description),
            deadline_at: Set(deadline_req.deadline_at.into()),
            is_milestone: Set(deadline_req.is_milestone),
            ..Default::default()
        };
        deadline.insert(&tx).await?;
    }

    // Create hackathon skills
    for skill_id_str in req.skills {
        let skill_id = skill_id_str.parse::<Uuid>()
            .map_err(|_| AppError::BadRequest("некорректный ID навыка".to_string()))?;
        let hs = hackathon_skill::ActiveModel {
            id: Set(Uuid::new_v4()),
            hackathon_id: Set(hackathon.id),
            skill_id: Set(skill_id),
            created_at: Set(Utc::now().into()),
        };
        hs.insert(&tx).await?;
    }

    tx.commit().await?;

    let response = build_hackathon_response(&state, hackathon.id).await?;

    Ok((StatusCode::CREATED, Json(response)))
}

async fn get_hackathon(
    State(state): State<SharedState>,
    claims: Option<Extension<Claims>>,
    Path(id): Path<Uuid>,
) -> Result<Json<HackathonResponse>, AppError> {
    let hackathon = hackathons::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Хакатон не найден".to_string()))?;

    // Hide non-approved hackathons from everyone except the owner and staff/superuser.
    let needs_visibility_check = hackathon.status == "pending" || hackathon.status == "rejected";
    if needs_visibility_check {
        let claims = claims.map(|Extension(c)| c);
        let mut allowed = false;

        if let Some(ref c) = claims {
            if c.is_staff || c.is_superuser {
                allowed = true;
            } else {
                let organizer = organizers::Entity::find()
                    .filter(organizers::Column::UserId.eq(c.sub))
                    .one(&state.db)
                    .await?;
                if let Some(org) = organizer {
                    if org.id == hackathon.organizer_id {
                        allowed = true;
                    }
                }
            }
        }

        if !allowed {
            return Err(AppError::NotFound("Хакатон не найден".to_string()));
        }
    }

    let response = build_hackathon_response(&state, id).await?;
    Ok(Json(response))
}

async fn update_hackathon(
    State(state): State<SharedState>,
    claims: Option<Extension<Claims>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateHackathonRequest>,
) -> Result<Json<HackathonResponse>, AppError> {
    let claims = claims
        .map(|Extension(claims)| claims)
        .ok_or(AppError::Unauthorized("Требуется авторизация".to_string()))?;

    let hackathon = hackathons::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Хакатон не найден".to_string()))?;

    // Get user's organizer
    let organizer = organizers::Entity::find()
        .filter(organizers::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    let can_edit = match organizer {
        Some(org) if org.id == hackathon.organizer_id => true,
        _ => false,
    };

    if !can_edit {
        return Err(AppError::Forbidden("Нет прав на редактирование".to_string()));
    }

    // Check if already published - editing published hackathons is forbidden
    if hackathon.is_published {
        return Err(AppError::Forbidden("Опубликованный хакатон нельзя редактировать".to_string()));
    }

    let was_rejected = hackathon.status == "rejected";
    let mut hackathon_active: hackathons::ActiveModel = hackathon.into();

    // If hackathon was rejected, reset status to pending for re-moderation
    if was_rejected {
        hackathon_active.status = Set("pending".to_string());
    }

    if let Some(title) = req.title {
        hackathon_active.title = Set(title);
    }
    if let Some(description) = req.description {
        hackathon_active.description = Set(Some(description));
    }
    if let Some(location_type) = req.location_type {
        hackathon_active.location_type = Set(location_type);
    }
    if let Some(city) = req.city {
        hackathon_active.city = Set(Some(city));
    }
    if let Some(venue) = req.venue {
        hackathon_active.venue = Set(Some(venue));
    }
    if let Some(max_participants) = req.max_participants {
        hackathon_active.max_participants = Set(Some(max_participants));
    }
    if let Some(banner_url) = req.banner_url {
        hackathon_active.banner_url = Set(Some(banner_url));
    }
    if let Some(registration_start) = req.registration_start {
        hackathon_active.registration_start = Set(registration_start.into());
    }
    if let Some(registration_end) = req.registration_end {
        hackathon_active.registration_end = Set(registration_end.into());
    }
    if let Some(event_start) = req.event_start {
        hackathon_active.event_start = Set(event_start.into());
    }
    if let Some(event_end) = req.event_end {
        hackathon_active.event_end = Set(event_end.into());
    }

    // New fields
    if let Some(contact_email) = req.contact_email {
        hackathon_active.contact_email = Set(Some(contact_email));
    }
    if let Some(website_url) = req.website_url {
        hackathon_active.website_url = Set(Some(website_url));
    }
    if let Some(social_links) = req.social_links {
        hackathon_active.social_links = Set(Some(social_links));
    }
    if let Some(prize_pool) = req.prize_pool {
        hackathon_active.prize_pool = Set(Some(prize_pool));
    }
    if let Some(prize_currency) = req.prize_currency {
        hackathon_active.prize_currency = Set(Some(prize_currency));
    }
    if let Some(prize_description) = req.prize_description {
        hackathon_active.prize_description = Set(Some(prize_description));
    }
    if let Some(requirements) = req.requirements {
        hackathon_active.requirements = Set(Some(requirements));
    }
    if let Some(team_size_min) = req.team_size_min {
        hackathon_active.team_size_min = Set(Some(team_size_min));
    }
    if let Some(team_size_max) = req.team_size_max {
        hackathon_active.team_size_max = Set(Some(team_size_max));
    }
    if let Some(age_restriction) = req.age_restriction {
        hackathon_active.age_restriction = Set(Some(age_restriction));
    }

    hackathon_active.updated_at = Set(Utc::now().into());

    let _hackathon = hackathon_active.update(&state.db).await?;

    // Update skills if provided
    if let Some(skill_ids) = req.skills {
        // Remove existing skills
        hackathon_skill::Entity::delete_many()
            .filter(hackathon_skill::Column::HackathonId.eq(id))
            .exec(&state.db)
            .await?;

        // Add new skills
        for skill_id_str in skill_ids {
            let skill_id = skill_id_str.parse::<Uuid>()
                .map_err(|_| AppError::BadRequest("некорректный ID навыка".to_string()))?;
            let hs = hackathon_skill::ActiveModel {
                id: Set(Uuid::new_v4()),
                hackathon_id: Set(id),
                skill_id: Set(skill_id),
                created_at: Set(Utc::now().into()),
            };
            hs.insert(&state.db).await?;
        }
    }

    let response = build_hackathon_response(&state, id).await?;
    Ok(Json(response))
}

async fn delete_hackathon(
    State(state): State<SharedState>,
    claims: Option<Extension<Claims>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let claims = claims
        .map(|Extension(claims)| claims)
        .ok_or(AppError::Unauthorized("Требуется авторизация".to_string()))?;

    let hackathon = hackathons::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Хакатон не найден".to_string()))?;

    let organizer = organizers::Entity::find()
        .filter(organizers::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    let is_owner = matches!(&organizer, Some(org) if org.id == hackathon.organizer_id);
    if !is_owner {
        return Err(AppError::Forbidden("Нет прав на удаление".to_string()));
    }

    // Approved hackathon with teams cannot be deleted — only cancelled
    if hackathon.status == "approved" {
        let team_count = teams::Entity::find()
            .filter(teams::Column::HackathonId.eq(hackathon.id))
            .count(&state.db)
            .await?;

        if team_count > 0 {
            return Err(AppError::BadRequest(
                "Нельзя удалить одобренный хакатон с зарегистрированными командами. Используйте отмену.".to_string(),
            ));
        }
    }

    if hackathon.status == "cancelled" {
        return Err(AppError::BadRequest(
            "Хакатон уже отменён и не может быть удалён организатором.".to_string(),
        ));
    }

    hackathons::Entity::delete_by_id(id).exec(&state.db).await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn cancel_hackathon(
    State(state): State<SharedState>,
    claims: Option<Extension<Claims>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let claims = claims
        .map(|Extension(claims)| claims)
        .ok_or(AppError::Unauthorized("Требуется авторизация".to_string()))?;

    let hackathon = hackathons::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Хакатон не найден".to_string()))?;

    let organizer = organizers::Entity::find()
        .filter(organizers::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    let is_owner = matches!(&organizer, Some(org) if org.id == hackathon.organizer_id);
    if !is_owner {
        return Err(AppError::Forbidden("Нет прав на отмену".to_string()));
    }

    if hackathon.status != "approved" {
        return Err(AppError::BadRequest(
            "Отменить можно только одобренный хакатон.".to_string(),
        ));
    }

    let now = Utc::now();
    let days_until_start = (hackathon.event_start.with_timezone(&Utc) - now).num_days();
    if days_until_start < 7 {
        return Err(AppError::BadRequest(
            "До начала хакатона меньше 7 дней. Отмена возможна только через администратора.".to_string(),
        ));
    }

    let mut active: hackathons::ActiveModel = hackathon.into();
    active.status = Set("cancelled".to_string());
    active.updated_at = Set(now.into());
    active.update(&state.db).await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn get_hackathon_participants(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<UserInfo>>, AppError> {
    let _hackathon = hackathons::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Хакатон не найден".to_string()))?;

    let participants = team_members::Entity::find()
        .inner_join(teams::Entity)
        .filter(teams::Column::HackathonId.eq(id))
        .inner_join(users::Entity)
        .all(&state.db)
        .await?;

    let user_ids: Vec<Uuid> = participants.iter().map(|p| p.user_id).collect();
    let users = users::Entity::find()
        .filter(users::Column::Id.is_in(user_ids))
        .all(&state.db)
        .await?;

    let result: Vec<UserInfo> = users
        .into_iter()
        .map(|u| UserInfo {
            id: u.id.to_string(),
            name: u.name,
            avatar_url: u.avatar_url,
        })
        .collect();

    Ok(Json(result))
}

async fn get_hackathon_teams(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<TeamSummary>>, AppError> {
    let teams = teams::Entity::find()
        .filter(teams::Column::HackathonId.eq(id))
        .all(&state.db)
        .await?;

    let team_ids: Vec<Uuid> = teams.iter().map(|t| t.id).collect();

    // Load member counts for all teams in one query
    let members = team_members::Entity::find()
        .filter(team_members::Column::TeamId.is_in(team_ids.clone()))
        .all(&state.db)
        .await?;

    use std::collections::HashMap;
    let mut member_counts: HashMap<Uuid, i64> = HashMap::new();
    for member in members {
        *member_counts.entry(member.team_id).or_insert(0) += 1;
    }

    let summaries: Vec<TeamSummary> = teams
        .into_iter()
        .map(|t| TeamSummary {
            id: t.id.to_string(),
            name: t.name,
            description: t.description,
            hackathon_id: t.hackathon_id.to_string(),
            track_name: None,
            status: t.status,
            member_count: *member_counts.get(&t.id).unwrap_or(&0),
        })
        .collect();

    Ok(Json(summaries))
}

async fn build_hackathon_response(
    state: &SharedState,
    id: Uuid,
) -> Result<HackathonResponse, AppError> {
    let hackathon = hackathons::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Хакатон не найден".to_string()))?;

    // Get organizer info
    let organizer = organizers::Entity::find_by_id(hackathon.organizer_id)
        .one(&state.db)
        .await?;

    let organizer_response = match organizer {
        Some(org) => Some(OrganizerResponse {
            id: org.id.to_string(),
            name: org.name,
            avatar_url: org.logo_url,
        }),
        None => None,
    };

    // Get tracks with team counts
    let tracks_data = tracks::Entity::find()
        .filter(tracks::Column::HackathonId.eq(id))
        .all(&state.db)
        .await?;

    let tracks: Vec<TrackResponse> = tracks_data
        .into_iter()
        .map(|t| TrackResponse {
            id: t.id.to_string(),
            name: t.name,
            description: t.description,
            prize_description: t.prize_description,
            max_teams: t.max_teams,
            team_count: 0,
        })
        .collect();

    // Get deadlines
    let deadlines_data = deadlines::Entity::find()
        .filter(deadlines::Column::HackathonId.eq(id))
        .order_by_asc(deadlines::Column::DeadlineAt)
        .all(&state.db)
        .await?;

    let deadlines: Vec<DeadlineResponse> = deadlines_data
        .into_iter()
        .map(|d| DeadlineResponse {
            id: d.id.to_string(),
            name: d.name,
            description: d.description,
            deadline_at: d.deadline_at.to_rfc3339(),
            is_milestone: d.is_milestone,
        })
        .collect();

    // Get hackathon skills
    let skills_data = hackathon_skill::Entity::find()
        .filter(hackathon_skill::Column::HackathonId.eq(id))
        .find_also_related(skills::Entity)
        .all(&state.db)
        .await?;

    let skills: Vec<HackathonSkillResponse> = skills_data
        .into_iter()
        .filter_map(|(_, skill_opt)| {
            skill_opt.map(|s| HackathonSkillResponse {
                id: s.id.to_string(),
                name: s.name,
                category: s.category,
            })
        })
        .collect();

    // Count teams
    let team_count = teams::Entity::find()
        .filter(teams::Column::HackathonId.eq(id))
        .count(&state.db)
        .await? as i64;

    // Count participants
    let participant_count = team_members::Entity::find()
        .inner_join(teams::Entity)
        .filter(teams::Column::HackathonId.eq(id))
        .count(&state.db)
        .await? as i64;

    let response = HackathonResponse {
        id: hackathon.id.to_string(),
        title: hackathon.title,
        description: hackathon.description,
        banner_url: hackathon.banner_url,
        location_type: hackathon.location_type,
        city: hackathon.city,
        venue: hackathon.venue,
        registration_start: hackathon.registration_start.to_rfc3339(),
        registration_end: hackathon.registration_end.to_rfc3339(),
        event_start: hackathon.event_start.to_rfc3339(),
        event_end: hackathon.event_end.to_rfc3339(),
        max_participants: hackathon.max_participants,
        organizer: organizer_response,
        is_published: hackathon.is_published,
        status: hackathon.status,
        tracks,
        deadlines,
        participant_count,
        team_count,
        contact_email: hackathon.contact_email,
        website_url: hackathon.website_url,
        social_links: hackathon.social_links,
        prize_pool: hackathon.prize_pool,
        prize_currency: hackathon.prize_currency,
        prize_description: hackathon.prize_description,
        requirements: hackathon.requirements,
        team_size_min: hackathon.team_size_min,
        team_size_max: hackathon.team_size_max,
        age_restriction: hackathon.age_restriction,
        skills,
    };

    Ok(response)
}

#[derive(Debug, serde::Serialize)]
struct UserInfo {
    id: String,
    name: String,
    avatar_url: Option<String>,
}

// ============== Rating Criteria Endpoints ==============

async fn get_hackathon_criteria(
    State(state): State<SharedState>,
    Path(hackathon_id): Path<Uuid>,
) -> Result<Json<CriteriaListResponse>, AppError> {
    // Verify hackathon exists
    let _hackathon = hackathons::Entity::find_by_id(hackathon_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Хакатон не найден".to_string()))?;

    let criteria = entity::rating_criteria::Entity::find()
        .filter(entity::rating_criteria::Column::HackathonId.eq(hackathon_id))
        .order_by_asc(entity::rating_criteria::Column::SortOrder)
        .all(&state.db)
        .await?;

    let response: Vec<CriteriaResponse> = criteria
        .into_iter()
        .map(|c| CriteriaResponse {
            id: c.id.to_string(),
            hackathon_id: c.hackathon_id.to_string(),
            name: c.name,
            description: c.description,
            weight: c.weight,
            max_score: c.max_score,
            sort_order: c.sort_order,
            created_at: c.created_at.to_rfc3339(),
            updated_at: c.updated_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(CriteriaListResponse { criteria: response }))
}

async fn create_criteria(
    State(state): State<SharedState>,
    claims: Option<Extension<Claims>>,
    Path(hackathon_id): Path<Uuid>,
    Json(req): Json<CreateCriteriaRequest>,
) -> Result<Json<CriteriaResponse>, AppError> {
    let claims = claims
        .map(|Extension(claims)| claims)
        .ok_or(AppError::Unauthorized("Требуется авторизация".to_string()))?;

    req.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Verify hackathon exists
    let hackathon = hackathons::Entity::find_by_id(hackathon_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Хакатон не найден".to_string()))?;

    // Verify user is the organizer
    let organizer = organizers::Entity::find()
        .filter(organizers::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    let is_organizer = match organizer {
        Some(org) if org.id == hackathon.organizer_id => true,
        _ => false,
    };

    if !is_organizer {
        return Err(AppError::Forbidden("Нет прав на редактирование".to_string()));
    }

    // Check total weight doesn't exceed 1.0
    let existing_criteria = entity::rating_criteria::Entity::find()
        .filter(entity::rating_criteria::Column::HackathonId.eq(hackathon_id))
        .all(&state.db)
        .await?;

    let total_weight: f32 = existing_criteria.iter().map(|c| c.weight).sum::<f32>() + req.weight;
    if total_weight > 1.0 {
        return Err(AppError::BadRequest(
            "Сумма весов критериев не может превышать 1.0".to_string(),
        ));
    }

    // Get max sort_order
    let max_sort_order = existing_criteria
        .iter()
        .map(|c| c.sort_order)
        .max()
        .unwrap_or(-1);

    let criteria = entity::rating_criteria::ActiveModel {
        id: Set(Uuid::new_v4()),
        hackathon_id: Set(hackathon_id),
        name: Set(req.name),
        description: Set(req.description),
        weight: Set(req.weight),
        max_score: Set(req.max_score),
        sort_order: Set(max_sort_order + 1),
        created_at: Set(Utc::now().into()),
        updated_at: Set(Utc::now().into()),
    };

    let criteria = criteria.insert(&state.db).await?;

    Ok(Json(CriteriaResponse {
        id: criteria.id.to_string(),
        hackathon_id: criteria.hackathon_id.to_string(),
        name: criteria.name,
        description: criteria.description,
        weight: criteria.weight,
        max_score: criteria.max_score,
        sort_order: criteria.sort_order,
        created_at: criteria.created_at.to_rfc3339(),
        updated_at: criteria.updated_at.to_rfc3339(),
    }))
}

async fn update_criteria(
    State(state): State<SharedState>,
    claims: Option<Extension<Claims>>,
    Path((hackathon_id, criteria_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<UpdateCriteriaRequest>,
) -> Result<Json<CriteriaResponse>, AppError> {
    let claims = claims
        .map(|Extension(claims)| claims)
        .ok_or(AppError::Unauthorized("Требуется авторизация".to_string()))?;

    // Verify hackathon exists
    let hackathon = hackathons::Entity::find_by_id(hackathon_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Хакатон не найден".to_string()))?;

    // Verify user is the organizer
    let organizer = organizers::Entity::find()
        .filter(organizers::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    let is_organizer = match organizer {
        Some(org) if org.id == hackathon.organizer_id => true,
        _ => false,
    };

    if !is_organizer {
        return Err(AppError::Forbidden("Нет прав на редактирование".to_string()));
    }

    let criteria = entity::rating_criteria::Entity::find_by_id(criteria_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Критерий не найден".to_string()))?;

    if criteria.hackathon_id != hackathon_id {
        return Err(AppError::BadRequest(
            "Критерий не принадлежит этому хакатону".to_string(),
        ));
    }

    // Check total weight if being updated
    if let Some(new_weight) = req.weight {
        let existing_criteria = entity::rating_criteria::Entity::find()
            .filter(entity::rating_criteria::Column::HackathonId.eq(hackathon_id))
            .filter(entity::rating_criteria::Column::Id.ne(criteria_id))
            .all(&state.db)
            .await?;

        let total_weight: f32 = existing_criteria.iter().map(|c| c.weight).sum::<f32>() + new_weight;
        if total_weight > 1.0 {
            return Err(AppError::BadRequest(
                "Сумма весов критериев не может превышать 1.0".to_string(),
            ));
        }
    }

    let mut active_model: entity::rating_criteria::ActiveModel = criteria.into();

    if let Some(name) = req.name {
        active_model.name = Set(name);
    }
    if let Some(description) = req.description {
        active_model.description = Set(Some(description));
    }
    if let Some(weight) = req.weight {
        active_model.weight = Set(weight);
    }
    active_model.updated_at = Set(Utc::now().into());

    let criteria = active_model.update(&state.db).await?;

    Ok(Json(CriteriaResponse {
        id: criteria.id.to_string(),
        hackathon_id: criteria.hackathon_id.to_string(),
        name: criteria.name,
        description: criteria.description,
        weight: criteria.weight,
        max_score: criteria.max_score,
        sort_order: criteria.sort_order,
        created_at: criteria.created_at.to_rfc3339(),
        updated_at: criteria.updated_at.to_rfc3339(),
    }))
}

async fn delete_criteria(
    State(state): State<SharedState>,
    claims: Option<Extension<Claims>>,
    Path((hackathon_id, criteria_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    let claims = claims
        .map(|Extension(claims)| claims)
        .ok_or(AppError::Unauthorized("Требуется авторизация".to_string()))?;

    // Verify hackathon exists
    let hackathon = hackathons::Entity::find_by_id(hackathon_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Хакатон не найден".to_string()))?;

    // Verify user is the organizer
    let organizer = organizers::Entity::find()
        .filter(organizers::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    let is_organizer = match organizer {
        Some(org) if org.id == hackathon.organizer_id => true,
        _ => false,
    };

    if !is_organizer {
        return Err(AppError::Forbidden("Нет прав на удаление".to_string()));
    }

    let criteria = entity::rating_criteria::Entity::find_by_id(criteria_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Критерий не найден".to_string()))?;

    if criteria.hackathon_id != hackathon_id {
        return Err(AppError::BadRequest(
            "Критерий не принадлежит этому хакатону".to_string(),
        ));
    }

    entity::rating_criteria::Entity::delete_by_id(criteria_id)
        .exec(&state.db)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn reorder_criteria(
    State(state): State<SharedState>,
    claims: Option<Extension<Claims>>,
    Path(hackathon_id): Path<Uuid>,
    Json(req): Json<ReorderCriteriaRequest>,
) -> Result<StatusCode, AppError> {
    let claims = claims
        .map(|Extension(claims)| claims)
        .ok_or(AppError::Unauthorized("Требуется авторизация".to_string()))?;

    // Verify hackathon exists
    let hackathon = hackathons::Entity::find_by_id(hackathon_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Хакатон не найден".to_string()))?;

    // Verify user is the organizer
    let organizer = organizers::Entity::find()
        .filter(organizers::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    let is_organizer = match organizer {
        Some(org) if org.id == hackathon.organizer_id => true,
        _ => false,
    };

    if !is_organizer {
        return Err(AppError::Forbidden("Нет прав на редактирование".to_string()));
    }

    // Update sort_order for each criteria
    let tx = state.db.begin().await?;
    for (i, criteria_id_str) in req.criteria_ids.iter().enumerate() {
        let criteria_id = criteria_id_str
            .parse::<Uuid>()
            .map_err(|_| AppError::BadRequest("Некорректный ID критерия".to_string()))?;

        let criteria = entity::rating_criteria::Entity::find_by_id(criteria_id)
            .one(&tx)
            .await?
            .ok_or(AppError::NotFound("Критерий не найден".to_string()))?;

        if criteria.hackathon_id != hackathon_id {
            return Err(AppError::BadRequest(
                "Критерий не принадлежит этому хакатону".to_string(),
            ));
        }

        let mut active: entity::rating_criteria::ActiveModel = criteria.into();
        active.sort_order = Set(i as i32);
        active.update(&tx).await?;
    }
    tx.commit().await?;

    Ok(StatusCode::OK)
}

// ============== Submission Rating Endpoints ==============

async fn get_submissions_with_ratings(
    State(state): State<SharedState>,
    claims: Option<Extension<Claims>>,
    Path(hackathon_id): Path<Uuid>,
) -> Result<Json<SubmissionsWithRatingsResponse>, AppError> {
    let claims = claims
        .map(|Extension(claims)| claims)
        .ok_or(AppError::Unauthorized("Требуется авторизация".to_string()))?;

    // Verify hackathon exists
    let hackathon = hackathons::Entity::find_by_id(hackathon_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Хакатон не найден".to_string()))?;

    // Verify user is the organizer
    let organizer = organizers::Entity::find()
        .filter(organizers::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    let is_organizer = match organizer {
        Some(org) if org.id == hackathon.organizer_id => true,
        _ => false,
    };

    if !is_organizer {
        return Err(AppError::Forbidden(
            "Нет прав на просмотр оценок".to_string(),
        ));
    }

    // Get all teams for this hackathon with their submissions
    let teams_data = teams::Entity::find()
        .filter(teams::Column::HackathonId.eq(hackathon_id))
        .all(&state.db)
        .await?;

    let mut submissions_with_ratings: Vec<SubmissionWithRating> = Vec::new();

    for team in teams_data {
        let submission = entity::submissions::Entity::find()
            .filter(entity::submissions::Column::TeamId.eq(team.id))
            .one(&state.db)
            .await?;

        if let Some(submission) = submission {
            // Get existing rating if any
            let rating = entity::submission_ratings::Entity::find()
                .filter(
                    entity::submission_ratings::Column::SubmissionId.eq(submission.id),
                )
                .find_also_related(entity::organizers::Entity)
                .one(&state.db)
                .await?;

            let rating_response = if let Some((rating_data, organizer_opt)) = rating {
                let scores = entity::submission_rating_scores::Entity::find()
                    .filter(
                        entity::submission_rating_scores::Column::SubmissionRatingId
                            .eq(rating_data.id),
                    )
                    .find_also_related(entity::rating_criteria::Entity)
                    .all(&state.db)
                    .await?;

                let score_details: Vec<ScoreDetail> = scores
                    .into_iter()
                    .map(|(score, criteria_opt)| {
                        let criteria = criteria_opt.unwrap();
                        ScoreDetail {
                            criteria_id: score.criteria_id.to_string(),
                            criteria_name: criteria.name,
                            score: score.score,
                            max_score: criteria.max_score,
                            weight: criteria.weight,
                            weighted_score: score.score as f32 * criteria.weight,
                        }
                    })
                    .collect();

                Some(RatingResponse {
                    id: rating_data.id.to_string(),
                    submission_id: rating_data.submission_id.to_string(),
                    organizer_id: rating_data.organizer_id.to_string(),
                    organizer_name: organizer_opt.map(|o| o.name).unwrap_or_default(),
                    total_score: rating_data.total_score,
                    feedback: rating_data.feedback,
                    is_final: rating_data.is_final,
                    scores: score_details,
                    created_at: rating_data.created_at.to_rfc3339(),
                    updated_at: rating_data.updated_at.to_rfc3339(),
                })
            } else {
                None
            };

            submissions_with_ratings.push(SubmissionWithRating {
                id: submission.id.to_string(),
                title: submission.title,
                description: submission.description,
                repo_url: submission.repo_url,
                demo_url: submission.demo_url,
                status: submission.status,
                submitted_at: submission.submitted_at.map(|t| t.to_rfc3339()),
                team: TeamBrief {
                    id: team.id.to_string(),
                    name: team.name,
                },
                rating: rating_response,
            });
        }
    }

    Ok(Json(SubmissionsWithRatingsResponse {
        submissions: submissions_with_ratings,
    }))
}

async fn create_rating(
    State(state): State<SharedState>,
    claims: Option<Extension<Claims>>,
    Path(hackathon_id): Path<Uuid>,
    Json(req): Json<CreateRatingRequest>,
) -> Result<Json<RatingResponse>, AppError> {
    let claims = claims
        .map(|Extension(claims)| claims)
        .ok_or(AppError::Unauthorized("Требуется авторизация".to_string()))?;

    req.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Verify hackathon exists and has ended
    let hackathon = hackathons::Entity::find_by_id(hackathon_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Хакатон не найден".to_string()))?;

    // Check if hackathon has ended
    if Utc::now() < hackathon.event_end {
        return Err(AppError::BadRequest(
            "Оценивание доступно только после окончания хакатона".to_string(),
        ));
    }

    // Verify user is the organizer
    let organizer = organizers::Entity::find()
        .filter(organizers::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    let (is_organizer, organizer_id) = match organizer {
        Some(org) if org.id == hackathon.organizer_id => (true, org.id),
        _ => (false, Uuid::nil()),
    };

    if !is_organizer {
        return Err(AppError::Forbidden("Нет прав на оценивание".to_string()));
    }

    // Verify submission exists and belongs to this hackathon
    let submission_id = req
        .submission_id
        .parse::<Uuid>()
        .map_err(|_| AppError::BadRequest("Некорректный ID решения".to_string()))?;

    let submission = entity::submissions::Entity::find_by_id(submission_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Решение не найдено".to_string()))?;

    let team = teams::Entity::find_by_id(submission.team_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Команда не найдена".to_string()))?;

    if team.hackathon_id != hackathon_id {
        return Err(AppError::BadRequest(
            "Решение не принадлежит этому хакатону".to_string(),
        ));
    }

    // Check if rating already exists
    let existing = entity::submission_ratings::Entity::find()
        .filter(entity::submission_ratings::Column::SubmissionId.eq(submission_id))
        .one(&state.db)
        .await?;

    if existing.is_some() {
        return Err(AppError::BadRequest(
            "Оценка для этого решения уже существует".to_string(),
        ));
    }

    // Get criteria for validation
    let criteria_list = entity::rating_criteria::Entity::find()
        .filter(entity::rating_criteria::Column::HackathonId.eq(hackathon_id))
        .all(&state.db)
        .await?;

    // Validate all criteria have scores
    let criteria_ids: std::collections::HashSet<String> = criteria_list
        .iter()
        .map(|c| c.id.to_string())
        .collect();

    let provided_ids: std::collections::HashSet<String> = req
        .scores
        .iter()
        .map(|s| s.criteria_id.clone())
        .collect();

    if provided_ids != criteria_ids {
        return Err(AppError::BadRequest(
            "Не все критерии имеют оценки".to_string(),
        ));
    }

    // Calculate total score
    let mut total_score: f32 = 0.0;
    let mut score_details: Vec<ScoreDetail> = Vec::new();

    for score_input in &req.scores {
        let criteria = criteria_list
            .iter()
            .find(|c| c.id.to_string() == score_input.criteria_id)
            .ok_or_else(|| AppError::BadRequest("Некорректный ID критерия".to_string()))?;

        if score_input.score < 0 || score_input.score > criteria.max_score {
            return Err(AppError::BadRequest(format!(
                "Оценка {} выходит за пределы допустимого диапазона 0-{}",
                score_input.score, criteria.max_score
            )));
        }

        let weighted_score = score_input.score as f32 * criteria.weight;
        total_score += weighted_score;

        score_details.push(ScoreDetail {
            criteria_id: criteria.id.to_string(),
            criteria_name: criteria.name.clone(),
            score: score_input.score,
            max_score: criteria.max_score,
            weight: criteria.weight,
            weighted_score,
        });
    }

    // Create rating in transaction
    let tx = state.db.begin().await?;

    let rating = entity::submission_ratings::ActiveModel {
        id: Set(Uuid::new_v4()),
        submission_id: Set(submission_id),
        organizer_id: Set(organizer_id),
        total_score: Set(total_score),
        feedback: Set(req.feedback),
        is_final: Set(req.is_final),
        created_at: Set(Utc::now().into()),
        updated_at: Set(Utc::now().into()),
    };

    let rating = rating.insert(&tx).await?;

    // Insert scores
    for score_input in req.scores {
        let criteria_id = score_input
            .criteria_id
            .parse::<Uuid>()
            .map_err(|_| AppError::BadRequest("Некорректный ID критерия".to_string()))?;

        let score = entity::submission_rating_scores::ActiveModel {
            id: Set(Uuid::new_v4()),
            submission_rating_id: Set(rating.id),
            criteria_id: Set(criteria_id),
            score: Set(score_input.score),
            created_at: Set(Utc::now().into()),
            updated_at: Set(Utc::now().into()),
        };
        score.insert(&tx).await?;
    }

    tx.commit().await?;

    let organizer = organizers::Entity::find_by_id(organizer_id)
        .one(&state.db)
        .await?;

    Ok(Json(RatingResponse {
        id: rating.id.to_string(),
        submission_id: rating.submission_id.to_string(),
        organizer_id: rating.organizer_id.to_string(),
        organizer_name: organizer.map(|o| o.name).unwrap_or_default(),
        total_score: rating.total_score,
        feedback: rating.feedback,
        is_final: rating.is_final,
        scores: score_details,
        created_at: rating.created_at.to_rfc3339(),
        updated_at: rating.updated_at.to_rfc3339(),
    }))
}

async fn update_rating(
    State(state): State<SharedState>,
    claims: Option<Extension<Claims>>,
    Path((hackathon_id, rating_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<UpdateRatingRequest>,
) -> Result<Json<RatingResponse>, AppError> {
    let claims = claims
        .map(|Extension(claims)| claims)
        .ok_or(AppError::Unauthorized("Требуется авторизация".to_string()))?;

    // Verify hackathon exists
    let hackathon = hackathons::Entity::find_by_id(hackathon_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Хакатон не найден".to_string()))?;

    // Verify user is the organizer
    let organizer = organizers::Entity::find()
        .filter(organizers::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    let is_organizer = match organizer {
        Some(org) if org.id == hackathon.organizer_id => true,
        _ => false,
    };

    if !is_organizer {
        return Err(AppError::Forbidden(
            "Нет прав на редактирование".to_string(),
        ));
    }

    let rating = entity::submission_ratings::Entity::find_by_id(rating_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Оценка не найдена".to_string()))?;

    // Verify submission belongs to this hackathon
    let submission = entity::submissions::Entity::find_by_id(rating.submission_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Решение не найдено".to_string()))?;

    let team = teams::Entity::find_by_id(submission.team_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Команда не найдена".to_string()))?;

    if team.hackathon_id != hackathon_id {
        return Err(AppError::BadRequest(
            "Оценка не принадлежит этому хакатону".to_string(),
        ));
    }

    // Get criteria
    let criteria_list = entity::rating_criteria::Entity::find()
        .filter(entity::rating_criteria::Column::HackathonId.eq(hackathon_id))
        .all(&state.db)
        .await?;

    // Calculate new total score if scores provided
    let (new_total_score, score_details) = if !req.scores.is_empty() {
        let mut total: f32 = 0.0;
        let mut details: Vec<ScoreDetail> = Vec::new();

        for score_input in &req.scores {
            let criteria = criteria_list
                .iter()
                .find(|c| c.id.to_string() == score_input.criteria_id)
                .ok_or_else(|| AppError::BadRequest("Некорректный ID критерия".to_string()))?;

            if score_input.score < 0 || score_input.score > criteria.max_score {
                return Err(AppError::BadRequest(format!(
                    "Оценка {} выходит за пределы диапазона 0-{}",
                    score_input.score, criteria.max_score
                )));
            }

            let weighted = score_input.score as f32 * criteria.weight;
            total += weighted;

            details.push(ScoreDetail {
                criteria_id: criteria.id.to_string(),
                criteria_name: criteria.name.clone(),
                score: score_input.score,
                max_score: criteria.max_score,
                weight: criteria.weight,
                weighted_score: weighted,
            });
        }
        (total, details)
    } else {
        (rating.total_score, Vec::new())
    };

    // Update rating in transaction
    let tx = state.db.begin().await?;

    let mut active: entity::submission_ratings::ActiveModel = rating.into();
    active.total_score = Set(new_total_score);
    if let Some(feedback) = req.feedback {
        active.feedback = Set(Some(feedback));
    }
    if let Some(is_final) = req.is_final {
        active.is_final = Set(is_final);
    }
    active.updated_at = Set(Utc::now().into());
    let rating = active.update(&tx).await?;

    // Update scores if provided
    if !req.scores.is_empty() {
        // Delete old scores
        entity::submission_rating_scores::Entity::delete_many()
            .filter(
                entity::submission_rating_scores::Column::SubmissionRatingId.eq(rating_id),
            )
            .exec(&tx)
            .await?;

        // Insert new scores
        for score_input in req.scores {
            let criteria_id = score_input
                .criteria_id
                .parse::<Uuid>()
                .map_err(|_| AppError::BadRequest("Некорректный ID критерия".to_string()))?;

            let score = entity::submission_rating_scores::ActiveModel {
                id: Set(Uuid::new_v4()),
                submission_rating_id: Set(rating_id),
                criteria_id: Set(criteria_id),
                score: Set(score_input.score),
                created_at: Set(Utc::now().into()),
                updated_at: Set(Utc::now().into()),
            };
            score.insert(&tx).await?;
        }
    }

    tx.commit().await?;

    // Get scores if not updated
    let final_scores = if score_details.is_empty() {
        let db_scores = entity::submission_rating_scores::Entity::find()
            .filter(
                entity::submission_rating_scores::Column::SubmissionRatingId.eq(rating_id),
            )
            .find_also_related(entity::rating_criteria::Entity)
            .all(&state.db)
            .await?;
        db_scores
            .into_iter()
            .map(|(score, criteria_opt)| {
                let criteria = criteria_opt.unwrap();
                ScoreDetail {
                    criteria_id: score.criteria_id.to_string(),
                    criteria_name: criteria.name,
                    score: score.score,
                    max_score: criteria.max_score,
                    weight: criteria.weight,
                    weighted_score: score.score as f32 * criteria.weight,
                }
            })
            .collect()
    } else {
        score_details
    };

    let organizer = organizers::Entity::find_by_id(rating.organizer_id)
        .one(&state.db)
        .await?;

    Ok(Json(RatingResponse {
        id: rating.id.to_string(),
        submission_id: rating.submission_id.to_string(),
        organizer_id: rating.organizer_id.to_string(),
        organizer_name: organizer.map(|o| o.name).unwrap_or_default(),
        total_score: rating.total_score,
        feedback: rating.feedback,
        is_final: rating.is_final,
        scores: final_scores,
        created_at: rating.created_at.to_rfc3339(),
        updated_at: rating.updated_at.to_rfc3339(),
    }))
}

async fn delete_rating(
    State(state): State<SharedState>,
    claims: Option<Extension<Claims>>,
    Path((hackathon_id, rating_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    let claims = claims
        .map(|Extension(claims)| claims)
        .ok_or(AppError::Unauthorized("Требуется авторизация".to_string()))?;

    // Verify hackathon exists
    let hackathon = hackathons::Entity::find_by_id(hackathon_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Хакатон не найден".to_string()))?;

    // Verify user is the organizer
    let organizer = organizers::Entity::find()
        .filter(organizers::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    let is_organizer = match organizer {
        Some(org) if org.id == hackathon.organizer_id => true,
        _ => false,
    };

    if !is_organizer {
        return Err(AppError::Forbidden("Нет прав на удаление".to_string()));
    }

    let rating = entity::submission_ratings::Entity::find_by_id(rating_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Оценка не найдена".to_string()))?;

    // Verify submission belongs to this hackathon
    let submission = entity::submissions::Entity::find_by_id(rating.submission_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Решение не найдено".to_string()))?;

    let team = teams::Entity::find_by_id(submission.team_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Команда не найдена".to_string()))?;

    if team.hackathon_id != hackathon_id {
        return Err(AppError::BadRequest(
            "Оценка не принадлежит этому хакатону".to_string(),
        ));
    }

    entity::submission_ratings::Entity::delete_by_id(rating_id)
        .exec(&state.db)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn get_public_ratings(
    State(state): State<SharedState>,
    Path(hackathon_id): Path<Uuid>,
) -> Result<Json<PublicRatingsResponse>, AppError> {
    // Verify hackathon exists
    let hackathon = hackathons::Entity::find_by_id(hackathon_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("Хакатон не найден".to_string()))?;

    // Get all ratings for this hackathon
    let ratings = entity::submission_ratings::Entity::find()
        .filter(entity::submission_ratings::Column::IsFinal.eq(true))
        .find_also_related(entity::submissions::Entity)
        .all(&state.db)
        .await?;

    let mut rating_entries: Vec<PublicRatingEntry> = Vec::new();

    for (rating, sub_opt) in ratings {
        if let Some(submission) = sub_opt {
            let team = teams::Entity::find_by_id(submission.team_id)
                .one(&state.db)
                .await?;

            if let Some(team) = team {
                // Verify team belongs to this hackathon
                if team.hackathon_id == hackathon_id {
                    rating_entries.push(PublicRatingEntry {
                        rank: 0, // Will be assigned after sorting
                        team_id: team.id.to_string(),
                        team_name: team.name,
                        submission_id: submission.id.to_string(),
                        submission_title: submission.title,
                        total_score: rating.total_score,
                        is_final: rating.is_final,
                        feedback: rating.feedback,
                    });
                }
            }
        }
    }

    // Sort by total_score descending and assign ranks
    rating_entries.sort_by(|a, b| b.total_score.partial_cmp(&a.total_score).unwrap_or(std::cmp::Ordering::Equal));
    for (i, entry) in rating_entries.iter_mut().enumerate() {
        entry.rank = i + 1;
    }

    Ok(Json(PublicRatingsResponse {
        hackathon_id: hackathon_id.to_string(),
        hackathon_title: hackathon.title,
        ratings: rating_entries,
    }))
}

use crate::models::rating::*;
use crate::middleware::auth::auth_middleware;

async fn list_my_hackathons(
    State(state): State<SharedState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<MyHackathonSummary>>, AppError> {
    // Find organizer for this user
    let organizer = organizers::Entity::find()
        .filter(organizers::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    let Some(org) = organizer else {
        return Ok(Json(vec![]));
    };

    // Get all hackathons for this organizer regardless of status
    let hackathons = hackathons::Entity::find()
        .filter(hackathons::Column::OrganizerId.eq(org.id))
        .order_by_desc(hackathons::Column::CreatedAt)
        .all(&state.db)
        .await?;

    let summaries: Vec<MyHackathonSummary> = hackathons
        .into_iter()
        .map(|h| MyHackathonSummary {
            id: h.id.to_string(),
            title: h.title,
            banner_url: h.banner_url,
            status: h.status,
            is_published: h.is_published,
            created_at: h.created_at.to_rfc3339(),
            event_start: h.event_start.to_rfc3339(),
            event_end: h.event_end.to_rfc3339(),
        })
        .collect();

    Ok(Json(summaries))
}
