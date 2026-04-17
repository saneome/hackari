use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Submissions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Submissions::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()"),
                    )
                    .col(
                        ColumnDef::new(Submissions::TeamId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Submissions::TrackId).uuid())
                    .col(ColumnDef::new(Submissions::Title).string_len(255).not_null())
                    .col(ColumnDef::new(Submissions::Description).text())
                    .col(ColumnDef::new(Submissions::RepoUrl).text())
                    .col(ColumnDef::new(Submissions::DemoUrl).text())
                    .col(
                        ColumnDef::new(Submissions::Status)
                            .string_len(20)
                            .not_null()
                            .default("draft"),
                    )
                    .col(ColumnDef::new(Submissions::SubmittedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Submissions::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .col(
                        ColumnDef::new(Submissions::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_submissions_team")
                            .from(Submissions::Table, Submissions::TeamId)
                            .to(Teams::Table, Teams::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_submissions_track")
                            .from(Submissions::Table, Submissions::TrackId)
                            .to(Tracks::Table, Tracks::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_submissions_team")
                    .table(Submissions::Table)
                    .col(Submissions::TeamId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().if_exists().name("idx_submissions_team").table(Submissions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().if_exists().table(Submissions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Submissions {
    Table,
    Id,
    TeamId,
    TrackId,
    Title,
    Description,
    RepoUrl,
    DemoUrl,
    Status,
    SubmittedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Teams {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Tracks {
    Table,
    Id,
}
