use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "hackathon_skills")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub hackathon_id: Uuid,
    pub skill_id: Uuid,
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
    #[sea_orm(
        belongs_to = "super::skills::Entity",
        from = "Column::SkillId",
        to = "super::skills::Column::Id"
    )]
    Skill,
}

impl Related<super::hackathons::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Hackathon.def()
    }
}

impl Related<super::skills::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Skill.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
