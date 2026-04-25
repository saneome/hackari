use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub github_url: Option<String>,
    pub telegram_username: Option<String>,
    pub is_verified: bool,
    pub organizer_terms_accepted_at: Option<DateTimeWithTimeZone>,
    pub is_staff: bool,
    pub is_superuser: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::organizers::Entity")]
    Organizers,
    #[sea_orm(has_many = "super::team_members::Entity")]
    TeamMembers,
    #[sea_orm(has_many = "super::sessions::Entity")]
    Sessions,
}

impl Related<super::organizers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organizers.def()
    }
}

impl Related<super::team_members::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TeamMembers.def()
    }
}

impl Related<super::sessions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sessions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
