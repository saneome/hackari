use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Organizers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Organizers::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()"),
                    )
                    .col(
                        ColumnDef::new(Organizers::UserId)
                            .uuid()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Organizers::Name).string_len(255).not_null())
                    .col(
                        ColumnDef::new(Organizers::Type)
                            .string_len(20)
                            .not_null()
                            .default("individual"),
                    )
                    .col(ColumnDef::new(Organizers::Description).text())
                    .col(ColumnDef::new(Organizers::WebsiteUrl).text())
                    .col(ColumnDef::new(Organizers::LogoUrl).text())
                    .col(ColumnDef::new(Organizers::Email).string_len(255).not_null())
                    .col(ColumnDef::new(Organizers::SocialLinks).json())
                    .col(
                        ColumnDef::new(Organizers::IsVerified)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Organizers::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .col(
                        ColumnDef::new(Organizers::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_organizers_user")
                            .from(Organizers::Table, Organizers::UserId)
                            .to(Users::Table, Users::Id)
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
                    .name("idx_organizers_user")
                    .table(Organizers::Table)
                    .col(Organizers::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx_organizers_user")
                    .table(Organizers::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().if_exists().table(Organizers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Organizers {
    Table,
    Id,
    UserId,
    Name,
    Type,
    Description,
    WebsiteUrl,
    LogoUrl,
    Email,
    SocialLinks,
    IsVerified,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
