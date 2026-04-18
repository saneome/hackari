use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Statement;
use sea_orm_migration::SchemaManager;
use sea_orm_migration::sea_orm::ConnectionTrait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create skills table
        manager
            .create_table(
                Table::create()
                    .table(Skills::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Skills::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()"),
                    )
                    .col(ColumnDef::new(Skills::Name).string_len(50).not_null().unique_key())
                    .col(ColumnDef::new(Skills::Category).string_len(50).not_null())
                    .col(ColumnDef::new(Skills::Icon).string_len(50))
                    .to_owned(),
            )
            .await?;

        // Create user_skills junction table
        manager
            .create_table(
                Table::create()
                    .table(UserSkills::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserSkills::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()"),
                    )
                    .col(ColumnDef::new(UserSkills::UserId).uuid().not_null())
                    .col(ColumnDef::new(UserSkills::SkillId).uuid().not_null())
                    .col(
                        ColumnDef::new(UserSkills::Level)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(UserSkills::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_skills_user_id")
                            .from(UserSkills::Table, UserSkills::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_skills_skill_id")
                            .from(UserSkills::Table, UserSkills::SkillId)
                            .to(Skills::Table, Skills::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create unique index to prevent duplicate skills per user
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_user_skills_unique")
                    .table(UserSkills::Table)
                    .col(UserSkills::UserId)
                    .col(UserSkills::SkillId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Insert default skills
        let skills = vec![
            // Languages
            ("JavaScript", "languages", "js"),
            ("TypeScript", "languages", "ts"),
            ("Python", "languages", "python"),
            ("Go", "languages", "go"),
            ("Rust", "languages", "rust"),
            ("Java", "languages", "java"),
            ("C++", "languages", "cpp"),
            ("C#", "languages", "csharp"),
            ("PHP", "languages", "php"),
            ("Ruby", "languages", "ruby"),
            ("Swift", "languages", "swift"),
            ("Kotlin", "languages", "kotlin"),
            // Frontend
            ("React", "frontend", "react"),
            ("Vue", "frontend", "vue"),
            ("Angular", "frontend", "angular"),
            ("Svelte", "frontend", "svelte"),
            ("Next.js", "frontend", "nextjs"),
            ("CSS/SCSS", "frontend", "css"),
            ("Tailwind", "frontend", "tailwind"),
            // Backend
            ("Node.js", "backend", "nodejs"),
            ("Express", "backend", "express"),
            ("Django", "backend", "django"),
            ("FastAPI", "backend", "fastapi"),
            ("Spring", "backend", "spring"),
            ("Laravel", "backend", "laravel"),
            // Database
            ("PostgreSQL", "database", "postgres"),
            ("MongoDB", "database", "mongodb"),
            ("Redis", "database", "redis"),
            ("MySQL", "database", "mysql"),
            ("Elasticsearch", "database", "elastic"),
            // DevOps
            ("Docker", "devops", "docker"),
            ("Kubernetes", "devops", "k8s"),
            ("CI/CD", "devops", "cicd"),
            ("AWS", "devops", "aws"),
            ("Terraform", "devops", "terraform"),
            ("Git", "devops", "git"),
            ("Linux", "devops", "linux"),
            ("Nginx", "devops", "nginx"),
            // Mobile
            ("React Native", "mobile", "reactnative"),
            ("Flutter", "mobile", "flutter"),
            ("iOS", "mobile", "ios"),
            ("Android", "mobile", "android"),
            // ML/Data
            ("Machine Learning", "ml", "ml"),
            ("TensorFlow", "ml", "tensorflow"),
            ("PyTorch", "ml", "pytorch"),
            ("Pandas", "ml", "pandas"),
            ("Data Science", "ml", "datascience"),
        // AI Tools
        ("GitHub Copilot", "ai-tools", "copilot"),
        ("Claude Code", "ai-tools", "claude-code"),
        ("OpenAI Codex", "ai-tools", "codex"),
        ("Cursor", "ai-tools", "cursor"),
        ("ChatGPT", "ai-tools", "chatgpt"),
        ("Claude", "ai-tools", "claude-ai"),
        ("Midjourney", "ai-tools", "midjourney"),
        ("Stable Diffusion", "ai-tools", "sd"),
        ("LangChain", "ai-tools", "langchain"),
        ("Hugging Face", "ai-tools", "huggingface"),
        ("Vibe Coding", "ai-tools", "vibe"),
            // Other
            ("UI/UX Design", "design", "uiux"),
            ("Figma", "design", "figma"),
            ("Product Management", "other", "product"),
            ("System Design", "other", "system"),
            ("Blockchain", "other", "blockchain"),
            ("Web3", "other", "web3"),
        ];

        for (name, category, icon) in skills {
            let sql = format!(
                "INSERT INTO skills (name, category, icon) VALUES ('{}', '{}', '{}') ON CONFLICT DO NOTHING",
                name, category, icon
            );
            let stmt = Statement::from_string(manager.get_database_backend(), sql);
            manager.get_connection().execute(stmt).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().if_exists().table(UserSkills::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().if_exists().table(Skills::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Skills {
    Table,
    Id,
    Name,
    Category,
    Icon,
}

#[derive(DeriveIden)]
enum UserSkills {
    Table,
    Id,
    UserId,
    SkillId,
    Level,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
