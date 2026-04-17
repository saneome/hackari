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
    pub organizer_id: Option<Uuid>,
    pub is_published: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::OrganizerId",
        to = "super::users::Column::Id"
    )]
    Organizer,
    #[sea_orm(has_many = "super::tracks::Entity")]
    Tracks,
    #[sea_orm(has_many = "super::teams::Entity")]
    Teams,
    #[sea_orm(has_many = "super::deadlines::Entity")]
    Deadlines,
}

impl Related<super::users::Entity> for Entity {
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
