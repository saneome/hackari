use axum::{
    extract::{ws::WebSocketUpgrade, State},
    response::Response,
};

use crate::services::{
    state::SharedState,
    websocket::handle_socket,
};

pub async fn handler(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}
