use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Teams::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Teams::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()"),
                    )
                    .col(
                        ColumnDef::new(Teams::HackathonId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Teams::TrackId).uuid())
                    .col(ColumnDef::new(Teams::Name).string_len(100).not_null())
                    .col(ColumnDef::new(Teams::Description).text())
                    .col(ColumnDef::new(Teams::RepoUrl).text())
                    .col(ColumnDef::new(Teams::DemoUrl).text())
                    .col(ColumnDef::new(Teams::PresentationUrl).text())
                    .col(
                        ColumnDef::new(Teams::Status)
                            .string_len(20)
                            .not_null()
                            .default("forming"),
                    )
                    .col(
                        ColumnDef::new(Teams::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .col(
                        ColumnDef::new(Teams::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_teams_hackathon")
                            .from(Teams::Table, Teams::HackathonId)
                            .to(Hackathons::Table, Hackathons::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_teams_track")
                            .from(Teams::Table, Teams::TrackId)
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
                    .name("idx_teams_hackathon")
                    .table(Teams::Table)
                    .col(Teams::HackathonId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_teams_track")
                    .table(Teams::Table)
                    .col(Teams::TrackId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().if_exists().name("idx_teams_track").table(Teams::Table).to_owned())
            .await?;
        manager
            .drop_index(Index::drop().if_exists().name("idx_teams_hackathon").table(Teams::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().if_exists().table(Teams::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Teams {
    Table,
    Id,
    HackathonId,
    TrackId,
    Name,
    Description,
    RepoUrl,
    DemoUrl,
    PresentationUrl,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Hackathons {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Tracks {
    Table,
    Id,
}
