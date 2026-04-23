use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "submission_rating_scores")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub submission_rating_id: Uuid,
    pub criteria_id: Uuid,
    pub score: i32,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::submission_ratings::Entity",
        from = "Column::SubmissionRatingId",
        to = "super::submission_ratings::Column::Id"
    )]
    SubmissionRating,
    #[sea_orm(
        belongs_to = "super::rating_criteria::Entity",
        from = "Column::CriteriaId",
        to = "super::rating_criteria::Column::Id"
    )]
    Criteria,
}

impl Related<super::submission_ratings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SubmissionRating.def()
    }
}

impl Related<super::rating_criteria::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Criteria.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
