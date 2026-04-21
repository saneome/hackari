use redis::aio::MultiplexedConnection;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::sync::broadcast;

use super::email::EmailService;

pub struct AppState {
 pub db: DatabaseConnection,
 pub redis: MultiplexedConnection,
 pub ws_tx: broadcast::Sender<String>,
 pub email_service: Arc<EmailService>,
}

impl AppState {
 pub fn new(db: DatabaseConnection, redis: MultiplexedConnection, smtp_user: &str, smtp_password: &str, from_email: &str, frontend_url: &str) -> Self {
  let (ws_tx, _rx) = broadcast::channel(100);
  let email_service = Arc::new(EmailService::new(smtp_user, smtp_password, from_email, frontend_url).expect("Failed to create email service"));
  Self { db, redis, ws_tx, email_service }
 }
}

pub type SharedState = Arc<AppState>;
