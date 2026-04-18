use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Statement;
use sea_orm_migration::sea_orm::ConnectionTrait;
use sea_orm_migration::SchemaManager;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Insert AI tools skills
        let skills = vec![
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
        // Delete AI tools skills
        let sql = "DELETE FROM skills WHERE category = 'ai-tools'";
        let stmt = Statement::from_string(manager.get_database_backend(), sql);
        manager.get_connection().execute(stmt).await?;

        Ok(())
    }
}
