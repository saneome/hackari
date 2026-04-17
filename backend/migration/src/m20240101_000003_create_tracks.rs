use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tracks::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tracks::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()"),
                    )
                    .col(
                        ColumnDef::new(Tracks::HackathonId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Tracks::Name).string_len(100).not_null())
                    .col(ColumnDef::new(Tracks::Description).text())
                    .col(ColumnDef::new(Tracks::PrizeDescription).text())
                    .col(ColumnDef::new(Tracks::MaxTeams).integer())
                    .col(
                        ColumnDef::new(Tracks::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tracks_hackathon")
                            .from(Tracks::Table, Tracks::HackathonId)
                            .to(Hackathons::Table, Hackathons::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_tracks_hackathon")
                    .table(Tracks::Table)
                    .col(Tracks::HackathonId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().if_exists().name("idx_tracks_hackathon").table(Tracks::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().if_exists().table(Tracks::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Tracks {
    Table,
    Id,
    HackathonId,
    Name,
    Description,
    PrizeDescription,
    MaxTeams,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Hackathons {
    Table,
    Id,
}
