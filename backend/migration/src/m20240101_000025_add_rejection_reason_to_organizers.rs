use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Organizers::Table)
                    .add_column(
                        ColumnDef::new(Organizers::RejectionReason)
                            .text()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Organizers::Table)
                    .drop_column(Organizers::RejectionReason)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Organizers {
    Table,
    RejectionReason,
}
