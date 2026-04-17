use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "teams")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub hackathon_id: Uuid,
    pub track_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub repo_url: Option<String>,
    pub demo_url: Option<String>,
    pub presentation_url: Option<String>,
    pub status: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::hackathons::Entity",
        from = "Column::HackathonId",
        to = "super::hackathons::Column::Id"
    )]
    Hackathon,
    #[sea_orm(
        belongs_to = "super::tracks::Entity",
        from = "Column::TrackId",
        to = "super::tracks::Column::Id"
    )]
    Track,
    #[sea_orm(has_many = "super::team_members::Entity")]
    TeamMembers,
    #[sea_orm(has_many = "super::submissions::Entity")]
    Submissions,
    #[sea_orm(has_many = "super::invitations::Entity")]
    Invitations,
}

impl Related<super::hackathons::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Hackathon.def()
    }
}

impl Related<super::tracks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Track.def()
    }
}

impl Related<super::team_members::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TeamMembers.def()
    }
}

impl Related<super::submissions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Submissions.def()
    }
}

impl Related<super::invitations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Invitations.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
