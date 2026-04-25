use axum::{
    Router,
    routing::get,
};

mod admin;
mod auth;
mod hackathons;
mod invitations;
mod organizers;
mod reports;
mod teams;
mod users;
mod websocket;

use crate::services::state::SharedState;

pub fn routes() -> Router<SharedState> {
    let admin_secret = std::env::var("ADMIN_SECRET_HASH")
        .unwrap_or_else(|_| "9f2c7b6e5a1d4c8fbbd2a0c3e7f1a6d9".to_string());

    Router::new()
        .nest("/auth", auth::routes())
        .nest("/users", users::routes())
        .nest("/hackathons", hackathons::routes())
        .nest("/teams", teams::routes())
        .nest("/invitations", invitations::routes())
        .nest("/organizers", organizers::routes())
        .nest("/reports", reports::routes())
        .nest(&format!("/admin/{}", admin_secret), admin::routes())
        .route("/ws", get(websocket::handler))
}