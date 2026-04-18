use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "organizers")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    #[sea_orm(column_name = "type")]
    pub type_: String,
    pub description: Option<String>,
    pub website_url: Option<String>,
    pub logo_url: Option<String>,
    pub email: String,
    pub social_links: Option<Json>,
    pub is_verified: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id"
    )]
    User,
    #[sea_orm(has_many = "super::hackathons::Entity")]
    Hackathons,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::hackathons::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Hackathons.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
