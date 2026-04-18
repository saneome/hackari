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
    pub fn new(db: DatabaseConnection, redis: MultiplexedConnection, email_api_key: &str, from_email: &str) -> Self {
        let (ws_tx, _rx) = broadcast::channel(100);
        let email_service = Arc::new(EmailService::new(email_api_key, from_email));
        Self {
            db,
            redis,
            ws_tx,
            email_service,
        }
    }
}

pub type SharedState = Arc<AppState>;
