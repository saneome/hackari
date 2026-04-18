use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "skills")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub name: String,
    pub category: String,
    pub icon: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_skills::Entity")]
    UserSkills,
}

impl Related<super::user_skills::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserSkills.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
