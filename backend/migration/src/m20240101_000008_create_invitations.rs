use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Invitations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Invitations::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()"),
                    )
                    .col(
                        ColumnDef::new(Invitations::TeamId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Invitations::InvitedBy)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Invitations::InvitedEmail).string_len(255).not_null())
                    .col(ColumnDef::new(Invitations::InvitedUserId).uuid())
                    .col(
                        ColumnDef::new(Invitations::Status)
                            .string_len(20)
                            .not_null()
                            .default("pending"),
                    )
                    .col(
                        ColumnDef::new(Invitations::Token)
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Invitations::ExpiresAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Invitations::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_invitations_team")
                            .from(Invitations::Table, Invitations::TeamId)
                            .to(Teams::Table, Teams::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_invitations_invited_by")
                            .from(Invitations::Table, Invitations::InvitedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_invitations_user")
                            .from(Invitations::Table, Invitations::InvitedUserId)
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
                    .name("idx_invitations_team")
                    .table(Invitations::Table)
                    .col(Invitations::TeamId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_invitations_token")
                    .table(Invitations::Table)
                    .col(Invitations::Token)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().if_exists().name("idx_invitations_token").table(Invitations::Table).to_owned())
            .await?;
        manager
            .drop_index(Index::drop().if_exists().name("idx_invitations_team").table(Invitations::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().if_exists().table(Invitations::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Invitations {
    Table,
    Id,
    TeamId,
    InvitedBy,
    InvitedEmail,
    InvitedUserId,
    Status,
    Token,
    ExpiresAt,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Teams {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
