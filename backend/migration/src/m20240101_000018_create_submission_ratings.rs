use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SubmissionRatings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SubmissionRatings::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()"),
                    )
                    .col(
                        ColumnDef::new(SubmissionRatings::SubmissionId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SubmissionRatings::OrganizerId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SubmissionRatings::TotalScore)
                            .float()
                            .not_null()
                            .default(0.0),
                    )
                    .col(ColumnDef::new(SubmissionRatings::Feedback).text())
                    .col(
                        ColumnDef::new(SubmissionRatings::IsFinal)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(SubmissionRatings::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .col(
                        ColumnDef::new(SubmissionRatings::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_submission_ratings_submission")
                            .from(SubmissionRatings::Table, SubmissionRatings::SubmissionId)
                            .to(Submissions::Table, Submissions::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_submission_ratings_organizer")
                            .from(SubmissionRatings::Table, SubmissionRatings::OrganizerId)
                            .to(Organizers::Table, Organizers::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_submission_ratings_submission")
                    .table(SubmissionRatings::Table)
                    .col(SubmissionRatings::SubmissionId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_submission_ratings_organizer")
                    .table(SubmissionRatings::Table)
                    .col(SubmissionRatings::OrganizerId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_submission_ratings_total_score")
                    .table(SubmissionRatings::Table)
                    .col(SubmissionRatings::TotalScore)
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
                    .name("idx_submission_ratings_total_score")
                    .table(SubmissionRatings::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx_submission_ratings_organizer")
                    .table(SubmissionRatings::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx_submission_ratings_submission")
                    .table(SubmissionRatings::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(SubmissionRatings::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SubmissionRatings {
    Table,
    Id,
    SubmissionId,
    OrganizerId,
    TotalScore,
    Feedback,
    IsFinal,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Submissions {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Organizers {
    Table,
    Id,
}
