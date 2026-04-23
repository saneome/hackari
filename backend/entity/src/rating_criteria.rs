use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "rating_criteria")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub hackathon_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub weight: f32,
    pub max_score: i32,
    pub sort_order: i32,
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
    #[sea_orm(has_many = "super::submission_rating_scores::Entity")]
    SubmissionRatingScores,
}

impl Related<super::hackathons::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Hackathon.def()
    }
}

impl Related<super::submission_rating_scores::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SubmissionRatingScores.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
