use axum::{
    Router,
    routing::get,
};

mod auth;
mod users;
mod hackathons;
mod teams;
mod invitations;
mod websocket;

use crate::services::state::SharedState;

pub fn routes() -> Router<SharedState> {
    Router::new()
        .nest("/auth", auth::routes())
        .nest("/users", users::routes())
        .nest("/hackathons", hackathons::routes())
        .nest("/teams", teams::routes())
        .nest("/invitations", invitations::routes())
        .route("/ws", get(websocket::handler))
}