use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(HackathonSkills::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(HackathonSkills::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()"),
                    )
                    .col(
                        ColumnDef::new(HackathonSkills::HackathonId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(HackathonSkills::SkillId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(HackathonSkills::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_hackathon_skills_hackathon")
                            .from(HackathonSkills::Table, HackathonSkills::HackathonId)
                            .to(Hackathons::Table, Hackathons::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_hackathon_skills_skill")
                            .from(HackathonSkills::Table, HackathonSkills::SkillId)
                            .to(Skills::Table, Skills::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Add unique constraint to prevent duplicate skills per hackathon
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_hackathon_skills_unique")
                    .table(HackathonSkills::Table)
                    .col(HackathonSkills::HackathonId)
                    .col(HackathonSkills::SkillId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_hackathon_skills_skill")
                    .table(HackathonSkills::Table)
                    .col(HackathonSkills::SkillId)
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
                    .name("idx_hackathon_skills_skill")
                    .table(HackathonSkills::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx_hackathon_skills_unique")
                    .table(HackathonSkills::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(HackathonSkills::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum HackathonSkills {
    Table,
    Id,
    HackathonId,
    SkillId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Hackathons {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Skills {
    Table,
    Id,
}
