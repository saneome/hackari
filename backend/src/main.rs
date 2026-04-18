use axum::{
    Router,
    routing::get,
};
use dotenvy::dotenv;
use migration::MigratorTrait;
use sea_orm::Database;
use std::sync::Arc;
use tower_http::{
    cors::{CorsLayer},
    trace::TraceLayer,
};
use tracing::info;
use tracing_subscriber;

mod api;
mod middleware;
mod models;
mod services;
mod utils;

use services::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    info!("Starting Hackari backend...");

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let email_api_key = std::env::var("RESEND_API_KEY")
        .expect("RESEND_API_KEY must be set");

    let from_email = std::env::var("FROM_EMAIL")
        .unwrap_or_else(|_| "noreply@hackari.ru".to_string());

    let db = Database::connect(&database_url).await?;
    info!("Connected to database");

    migration::Migrator::up(&db, None).await?;
    info!("Migrations applied");

    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    let redis_client = redis::Client::open(redis_url)?;
    let redis = redis_client.get_multiplexed_tokio_connection().await?;
    info!("Connected to Redis");

    let state = Arc::new(AppState::new(db, redis, &email_api_key, &from_email));

    let cors = CorsLayer::new()
        .allow_origin(["http://localhost:5173".parse()?, "http://localhost:3000".parse()?])
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
            axum::http::Method::PATCH,
            axum::http::Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
            axum::http::header::ACCEPT,
        ])
        .allow_credentials(true);

    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api", api::routes())
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
