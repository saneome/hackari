use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tracks")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub hackathon_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub prize_description: Option<String>,
    pub max_teams: Option<i32>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::hackathons::Entity",
        from = "Column::HackathonId",
        to = "super::hackathons::Column::Id"
    )]
    Hackathon,
    #[sea_orm(has_many = "super::teams::Entity")]
    Teams,
    #[sea_orm(has_many = "super::submissions::Entity")]
    Submissions,
}

impl Related<super::hackathons::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Hackathon.def()
    }
}

impl Related<super::teams::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Teams.def()
    }
}

impl Related<super::submissions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Submissions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
