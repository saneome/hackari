use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "hackathons")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub banner_url: Option<String>,
    pub location_type: String,
    pub city: Option<String>,
    pub venue: Option<String>,
    pub registration_start: DateTimeWithTimeZone,
    pub registration_end: DateTimeWithTimeZone,
    pub event_start: DateTimeWithTimeZone,
    pub event_end: DateTimeWithTimeZone,
    pub max_participants: Option<i32>,
    #[sea_orm(column_name = "new_organizer_id")]
    pub organizer_id: Uuid,
    pub is_published: bool,
    pub contact_email: Option<String>,
    pub website_url: Option<String>,
    pub social_links: Option<Json>,
    pub prize_pool: Option<String>,
    pub prize_currency: Option<String>,
    pub prize_description: Option<String>,
    pub requirements: Option<String>,
    pub team_size_min: Option<i32>,
    pub team_size_max: Option<i32>,
    pub age_restriction: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::organizers::Entity",
        from = "Column::OrganizerId",
        to = "super::organizers::Column::Id"
    )]
    Organizer,
    #[sea_orm(has_many = "super::tracks::Entity")]
    Tracks,
    #[sea_orm(has_many = "super::teams::Entity")]
    Teams,
    #[sea_orm(has_many = "super::deadlines::Entity")]
    Deadlines,
    #[sea_orm(has_many = "super::hackathon_skill::Entity")]
    HackathonSkills,
}

impl Related<super::organizers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organizer.def()
    }
}

impl Related<super::tracks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tracks.def()
    }
}

impl Related<super::teams::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Teams.def()
    }
}

impl Related<super::deadlines::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Deadlines.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
