use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RatingCriteria::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RatingCriteria::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()"),
                    )
                    .col(
                        ColumnDef::new(RatingCriteria::HackathonId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RatingCriteria::Name)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(ColumnDef::new(RatingCriteria::Description).text())
                    .col(
                        ColumnDef::new(RatingCriteria::Weight)
                            .float()
                            .not_null()
                            .default(0.2),
                    )
                    .col(
                        ColumnDef::new(RatingCriteria::MaxScore)
                            .integer()
                            .not_null()
                            .default(10),
                    )
                    .col(
                        ColumnDef::new(RatingCriteria::SortOrder)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(RatingCriteria::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .col(
                        ColumnDef::new(RatingCriteria::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_rating_criteria_hackathon")
                            .from(RatingCriteria::Table, RatingCriteria::HackathonId)
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
                    .name("idx_rating_criteria_hackathon")
                    .table(RatingCriteria::Table)
                    .col(RatingCriteria::HackathonId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_rating_criteria_sort_order")
                    .table(RatingCriteria::Table)
                    .col(RatingCriteria::SortOrder)
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
                    .name("idx_rating_criteria_sort_order")
                    .table(RatingCriteria::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx_rating_criteria_hackathon")
                    .table(RatingCriteria::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(RatingCriteria::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RatingCriteria {
    Table,
    Id,
    HackathonId,
    Name,
    Description,
    Weight,
    MaxScore,
    SortOrder,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Hackathons {
    Table,
    Id,
}
