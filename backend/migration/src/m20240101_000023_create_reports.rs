use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create reports table
        manager
            .create_table(
                Table::create()
                    .table(Reports::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Reports::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()"),
                    )
                    .col(
                        ColumnDef::new(Reports::ReporterId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Reports::TargetType)
                            .string_len(20)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Reports::TargetId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Reports::Reason)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Reports::Description)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Reports::Status)
                            .string_len(20)
                            .not_null()
                            .default("open"),
                    )
                    .col(
                        ColumnDef::new(Reports::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .col(
                        ColumnDef::new(Reports::ResolvedBy)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Reports::ResolvedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Reports::ResolutionNote)
                            .text()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_reports_reporter_id")
                            .from(Reports::Table, Reports::ReporterId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_reports_resolved_by")
                            .from(Reports::Table, Reports::ResolvedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index on status
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_reports_status")
                    .table(Reports::Table)
                    .col(Reports::Status)
                    .to_owned(),
            )
            .await?;

        // Create composite index on target_type and target_id
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_reports_target")
                    .table(Reports::Table)
                    .col(Reports::TargetType)
                    .col(Reports::TargetId)
                    .to_owned(),
            )
            .await?;

        // Create index on reporter_id
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_reports_reporter_id")
                    .table(Reports::Table)
                    .col(Reports::ReporterId)
                    .to_owned(),
            )
            .await?;

        // Create index on created_at
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_reports_created_at")
                    .table(Reports::Table)
                    .col(Reports::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // Create index on resolved_by
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_reports_resolved_by")
                    .table(Reports::Table)
                    .col(Reports::ResolvedBy)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop indexes
        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx_reports_resolved_by")
                    .table(Reports::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx_reports_created_at")
                    .table(Reports::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx_reports_reporter_id")
                    .table(Reports::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx_reports_target")
                    .table(Reports::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx_reports_status")
                    .table(Reports::Table)
                    .to_owned(),
            )
            .await?;

        // Drop table
        manager
            .drop_table(Table::drop().if_exists().table(Reports::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Reports {
    Table,
    Id,
    ReporterId,
    TargetType,
    TargetId,
    Reason,
    Description,
    Status,
    CreatedAt,
    ResolvedBy,
    ResolvedAt,
    ResolutionNote,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
