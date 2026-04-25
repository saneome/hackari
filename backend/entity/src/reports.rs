use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "reports")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub reporter_id: Uuid,
    pub target_type: String,
    pub target_id: Uuid,
    pub reason: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: DateTimeWithTimeZone,
    pub resolved_by: Option<Uuid>,
    pub resolved_at: Option<DateTimeWithTimeZone>,
    pub resolution_note: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::ReporterId",
        to = "super::users::Column::Id"
    )]
    Reporter,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::ResolvedBy",
        to = "super::users::Column::Id"
    )]
    Resolver,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Reporter.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
