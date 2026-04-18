use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add new columns to hackathons table
        manager
            .alter_table(
                Table::alter()
                    .table(Hackathons::Table)
                    .add_column(ColumnDef::new(Hackathons::ContactEmail).string_len(255))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Hackathons::Table)
                    .add_column(ColumnDef::new(Hackathons::WebsiteUrl).text())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Hackathons::Table)
                    .add_column(ColumnDef::new(Hackathons::SocialLinks).json())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Hackathons::Table)
                    .add_column(ColumnDef::new(Hackathons::PrizePool).string_len(100))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Hackathons::Table)
                    .add_column(
                        ColumnDef::new(Hackathons::PrizeCurrency)
                            .string_len(10)
                            .default("RUB"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Hackathons::Table)
                    .add_column(ColumnDef::new(Hackathons::PrizeDescription).text())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Hackathons::Table)
                    .add_column(ColumnDef::new(Hackathons::Requirements).text())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Hackathons::Table)
                    .add_column(ColumnDef::new(Hackathons::TeamSizeMin).integer())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Hackathons::Table)
                    .add_column(ColumnDef::new(Hackathons::TeamSizeMax).integer())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Hackathons::Table)
                    .add_column(ColumnDef::new(Hackathons::AgeRestriction).string_len(10))
                    .to_owned(),
            )
            .await?;

        // Drop old foreign key
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_hackathons_organizer")
                    .table(Hackathons::Table)
                    .to_owned(),
            )
            .await?;

        // Add new foreign key to organizers table
        manager
            .alter_table(
                Table::alter()
                    .table(Hackathons::Table)
                    .add_column(
                        ColumnDef::new(Hackathons::NewOrganizerId)
                            .uuid()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_hackathons_organizer_new")
                    .from(Hackathons::Table, Hackathons::NewOrganizerId)
                    .to(Organizers::Table, Organizers::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_hackathons_organizer_new")
                    .table(Hackathons::Table)
                    .col(Hackathons::NewOrganizerId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Remove new foreign key
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_hackathons_organizer_new")
                    .table(Hackathons::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx_hackathons_organizer_new")
                    .table(Hackathons::Table)
                    .to_owned(),
            )
            .await?;

        // Drop new columns
        manager
            .alter_table(
                Table::alter()
                    .table(Hackathons::Table)
                    .drop_column(Hackathons::NewOrganizerId)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Hackathons::Table)
                    .drop_column(Hackathons::ContactEmail)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Hackathons::Table)
                    .drop_column(Hackathons::WebsiteUrl)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Hackathons::Table)
                    .drop_column(Hackathons::SocialLinks)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Hackathons::Table)
                    .drop_column(Hackathons::PrizePool)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Hackathons::Table)
                    .drop_column(Hackathons::PrizeCurrency)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Hackathons::Table)
                    .drop_column(Hackathons::PrizeDescription)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Hackathons::Table)
                    .drop_column(Hackathons::Requirements)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Hackathons::Table)
                    .drop_column(Hackathons::TeamSizeMin)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Hackathons::Table)
                    .drop_column(Hackathons::TeamSizeMax)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Hackathons::Table)
                    .drop_column(Hackathons::AgeRestriction)
                    .to_owned(),
            )
            .await?;

        // Restore old foreign key
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_hackathons_organizer")
                    .from(Hackathons::Table, Hackathons::OldOrganizerId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::SetNull)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Hackathons {
    Table,
    ContactEmail,
    WebsiteUrl,
    SocialLinks,
    PrizePool,
    PrizeCurrency,
    PrizeDescription,
    Requirements,
    TeamSizeMin,
    TeamSizeMax,
    AgeRestriction,
    NewOrganizerId,
    OldOrganizerId,
}

#[derive(DeriveIden)]
enum Organizers {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
