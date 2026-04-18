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
        .route("/:id", get(get_hackathon).put(update_hackathon).delete(delete_hackathon))
        .route("/:id/participants", get(get_hackathon_participants))
        .route("/:id/teams", get(get_hackathon_teams))
        .layer(middleware::from_fn(optional_auth_middleware))
}

async fn list_hackathons(
    State(state): State<SharedState>,
) -> Result<Json<HackathonListResponse>, AppError> {
    let hackathons = hackathons::Entity::find()
        .filter(hackathons::Column::IsPublished.eq(true))
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

    let organizer_id = match organizer {
        Some(org) => org.id,
        None => {
            // Auto-create organizer from user info
            let user = users::Entity::find_by_id(claims.sub)
                .one(&state.db)
                .await?
                .ok_or(AppError::NotFound("Пользователь не найден".to_string()))?;

            let new_organizer = organizers::ActiveModel {
                id: Set(Uuid::new_v4()),
                user_id: Set(claims.sub),
                name: Set(user.name.clone()),
                type_: Set("individual".to_string()),
                description: Set(None),
                website_url: Set(None),
                logo_url: Set(user.avatar_url),
                email: Set(user.email),
                social_links: Set(None),
                is_verified: Set(false),
                created_at: Set(Utc::now().into()),
                updated_at: Set(Utc::now().into()),
            };

            let org = new_organizer.insert(&state.db).await?;
            org.id
        }
    };

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
        is_published: Set(true), // Auto-publish for MVP
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
    Path(id): Path<Uuid>,
) -> Result<Json<HackathonResponse>, AppError> {
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

    let mut hackathon_active: hackathons::ActiveModel = hackathon.into();

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

    // Get user's organizer
    let organizer = organizers::Entity::find()
        .filter(organizers::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await?;

    let can_delete = match organizer {
        Some(org) if org.id == hackathon.organizer_id => true,
        _ => false,
    };

    if !can_delete {
        return Err(AppError::Forbidden("Нет прав на удаление".to_string()));
    }

    hackathons::Entity::delete_by_id(id).exec(&state.db).await?;

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

    let summaries: Vec<TeamSummary> = teams
        .into_iter()
        .map(|t| TeamSummary {
            id: t.id.to_string(),
            name: t.name,
            description: t.description,
            hackathon_id: t.hackathon_id.to_string(),
            track_name: None,
            status: t.status,
            member_count: 0,
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
