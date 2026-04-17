use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Hackathons::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Hackathons::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()"),
                    )
                    .col(ColumnDef::new(Hackathons::Title).string_len(255).not_null())
                    .col(ColumnDef::new(Hackathons::Description).text())
                    .col(ColumnDef::new(Hackathons::BannerUrl).text())
                    .col(
                        ColumnDef::new(Hackathons::LocationType)
                            .string_len(20)
                            .not_null()
                            .default("online"),
                    )
                    .col(ColumnDef::new(Hackathons::City).string_len(100))
                    .col(ColumnDef::new(Hackathons::Venue).text())
                    .col(
                        ColumnDef::new(Hackathons::RegistrationStart)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Hackathons::RegistrationEnd)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Hackathons::EventStart)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Hackathons::EventEnd)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Hackathons::MaxParticipants).integer())
                    .col(ColumnDef::new(Hackathons::OrganizerId).uuid())
                    .col(
                        ColumnDef::new(Hackathons::IsPublished)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Hackathons::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .col(
                        ColumnDef::new(Hackathons::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_hackathons_organizer")
                            .from(Hackathons::Table, Hackathons::OrganizerId)
                            .to(Users::Table, Users::Id)
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
                    .name("idx_hackathons_dates")
                    .table(Hackathons::Table)
                    .col(Hackathons::RegistrationStart)
                    .col(Hackathons::RegistrationEnd)
                    .col(Hackathons::EventStart)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_hackathons_organizer")
                    .table(Hackathons::Table)
                    .col(Hackathons::OrganizerId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().if_exists().name("idx_hackathons_organizer").table(Hackathons::Table).to_owned())
            .await?;
        manager
            .drop_index(Index::drop().if_exists().name("idx_hackathons_dates").table(Hackathons::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().if_exists().table(Hackathons::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Hackathons {
    Table,
    Id,
    Title,
    Description,
    BannerUrl,
    LocationType,
    City,
    Venue,
    RegistrationStart,
    RegistrationEnd,
    EventStart,
    EventEnd,
    MaxParticipants,
    OrganizerId,
    IsPublished,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
