use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Deadlines::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Deadlines::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()"),
                    )
                    .col(
                        ColumnDef::new(Deadlines::HackathonId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Deadlines::Name).string_len(100).not_null())
                    .col(ColumnDef::new(Deadlines::Description).text())
                    .col(
                        ColumnDef::new(Deadlines::DeadlineAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Deadlines::IsMilestone)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Deadlines::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_deadlines_hackathon")
                            .from(Deadlines::Table, Deadlines::HackathonId)
                            .to(Hackathons::Table, Hackathons::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().if_exists().table(Deadlines::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Deadlines {
    Table,
    Id,
    HackathonId,
    Name,
    Description,
    DeadlineAt,
    IsMilestone,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Hackathons {
    Table,
    Id,
}
