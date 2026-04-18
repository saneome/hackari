use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PasswordResetCodes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PasswordResetCodes::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()"),
                    )
                    .col(
                        ColumnDef::new(PasswordResetCodes::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PasswordResetCodes::Code)
                            .string_len(6)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PasswordResetCodes::Used)
                            .boolean()
                            .not_null()
                            .extra("DEFAULT false"),
                    )
                    .col(
                        ColumnDef::new(PasswordResetCodes::ExpiresAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PasswordResetCodes::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_reset_codes_user")
                            .from(PasswordResetCodes::Table, PasswordResetCodes::UserId)
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
                    .name("idx_reset_codes_user")
                    .table(PasswordResetCodes::Table)
                    .col(PasswordResetCodes::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_reset_codes_code")
                    .table(PasswordResetCodes::Table)
                    .col(PasswordResetCodes::Code)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_reset_codes_expires")
                    .table(PasswordResetCodes::Table)
                    .col(PasswordResetCodes::ExpiresAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().if_exists().name("idx_reset_codes_expires").table(PasswordResetCodes::Table).to_owned())
            .await?;
        manager
            .drop_index(Index::drop().if_exists().name("idx_reset_codes_code").table(PasswordResetCodes::Table).to_owned())
            .await?;
        manager
            .drop_index(Index::drop().if_exists().name("idx_reset_codes_user").table(PasswordResetCodes::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().if_exists().table(PasswordResetCodes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PasswordResetCodes {
    Table,
    Id,
    UserId,
    Code,
    Used,
    ExpiresAt,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
