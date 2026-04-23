use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SubmissionRatingScores::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SubmissionRatingScores::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()"),
                    )
                    .col(
                        ColumnDef::new(SubmissionRatingScores::SubmissionRatingId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SubmissionRatingScores::CriteriaId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SubmissionRatingScores::Score)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(SubmissionRatingScores::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .col(
                        ColumnDef::new(SubmissionRatingScores::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_submission_rating_scores_rating")
                            .from(
                                SubmissionRatingScores::Table,
                                SubmissionRatingScores::SubmissionRatingId,
                            )
                            .to(SubmissionRatings::Table, SubmissionRatings::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_submission_rating_scores_criteria")
                            .from(SubmissionRatingScores::Table, SubmissionRatingScores::CriteriaId)
                            .to(RatingCriteria::Table, RatingCriteria::Id)
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
                    .name("idx_submission_rating_scores_rating")
                    .table(SubmissionRatingScores::Table)
                    .col(SubmissionRatingScores::SubmissionRatingId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_submission_rating_scores_criteria")
                    .table(SubmissionRatingScores::Table)
                    .col(SubmissionRatingScores::CriteriaId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_submission_rating_scores_unique")
                    .table(SubmissionRatingScores::Table)
                    .col(SubmissionRatingScores::SubmissionRatingId)
                    .col(SubmissionRatingScores::CriteriaId)
                    .unique()
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
                    .name("idx_submission_rating_scores_unique")
                    .table(SubmissionRatingScores::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx_submission_rating_scores_criteria")
                    .table(SubmissionRatingScores::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx_submission_rating_scores_rating")
                    .table(SubmissionRatingScores::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(SubmissionRatingScores::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum SubmissionRatingScores {
    Table,
    Id,
    SubmissionRatingId,
    CriteriaId,
    Score,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum SubmissionRatings {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum RatingCriteria {
    Table,
    Id,
}
