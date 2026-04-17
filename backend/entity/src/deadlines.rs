use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "deadlines")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub hackathon_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub deadline_at: DateTimeWithTimeZone,
    pub is_milestone: bool,
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
}

impl Related<super::hackathons::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Hackathon.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
