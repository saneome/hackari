use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct AppState {
    pub db: DatabaseConnection,
    pub ws_tx: broadcast::Sender<String>,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        let (ws_tx, _rx) = broadcast::channel(100);
        Self { db, ws_tx }
    }
}

pub type SharedState = Arc<AppState>;
