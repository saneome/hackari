use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Delete unverified users
        // This will cascade to related records (sessions, team_members, organizers)
        // due to foreign key constraints with ON DELETE CASCADE
        manager
            .get_connection()
            .execute_unprepared(
                "DELETE FROM users WHERE is_verified = false",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Cannot restore deleted users
        Ok(())
    }
}
