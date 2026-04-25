use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add status column to hackathons
        manager
            .alter_table(
                Table::alter()
                    .table(Hackathons::Table)
                    .add_column(
                        ColumnDef::new(Hackathons::Status)
                            .string_len(20)
                            .not_null()
                            .default("pending"),
                    )
                    .to_owned(),
            )
            .await?;

        // Update existing hackathons that are published to approved status
        let db = manager.get_connection();
        let stmt = Statement::from_string(
            manager.get_database_backend(),
            "UPDATE hackathons SET status = 'approved' WHERE is_published = true".to_string(),
        );
        db.execute(stmt).await?;

        // Create index on status
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_hackathons_status")
                    .table(Hackathons::Table)
                    .col(Hackathons::Status)
                    .to_owned(),
            )
            .await?;

        // Create composite index on status and created_at for moderation queue
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_hackathons_status_created_at")
                    .table(Hackathons::Table)
                    .col(Hackathons::Status)
                    .col(Hackathons::CreatedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop indexes
        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx_hackathons_status_created_at")
                    .table(Hackathons::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx_hackathons_status")
                    .table(Hackathons::Table)
                    .to_owned(),
            )
            .await?;

        // Drop status column
        manager
            .alter_table(
                Table::alter()
                    .table(Hackathons::Table)
                    .drop_column(Hackathons::Status)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Hackathons {
    Table,
    Status,
    IsPublished,
    CreatedAt,
}
