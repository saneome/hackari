use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "submission_ratings")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub submission_id: Uuid,
    pub organizer_id: Uuid,
    pub total_score: f32,
    pub feedback: Option<String>,
    pub is_final: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::submissions::Entity",
        from = "Column::SubmissionId",
        to = "super::submissions::Column::Id"
    )]
    Submission,
    #[sea_orm(
        belongs_to = "super::organizers::Entity",
        from = "Column::OrganizerId",
        to = "super::organizers::Column::Id"
    )]
    Organizer,
    #[sea_orm(has_many = "super::submission_rating_scores::Entity")]
    SubmissionRatingScores,
}

impl Related<super::submissions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Submission.def()
    }
}

impl Related<super::organizers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organizer.def()
    }
}

impl Related<super::submission_rating_scores::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SubmissionRatingScores.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
