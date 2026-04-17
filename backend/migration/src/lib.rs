pub use sea_orm_migration::prelude::*;

mod m20240101_000001_create_users;
mod m20240101_000002_create_hackathons;
mod m20240101_000003_create_tracks;
mod m20240101_000004_create_teams;
mod m20240101_000005_create_team_members;
mod m20240101_000006_create_submissions;
mod m20240101_000007_create_deadlines;
mod m20240101_000008_create_invitations;
mod m20240101_000009_create_sessions;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240101_000001_create_users::Migration),
            Box::new(m20240101_000002_create_hackathons::Migration),
            Box::new(m20240101_000003_create_tracks::Migration),
            Box::new(m20240101_000004_create_teams::Migration),
            Box::new(m20240101_000005_create_team_members::Migration),
            Box::new(m20240101_000006_create_submissions::Migration),
            Box::new(m20240101_000007_create_deadlines::Migration),
            Box::new(m20240101_000008_create_invitations::Migration),
            Box::new(m20240101_000009_create_sessions::Migration),
        ]
    }
}
