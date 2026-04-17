use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};
use serde::{Deserialize, Serialize};

use crate::services::state::SharedState;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    #[serde(rename = "team_invite")]
    TeamInvite { team_id: String, team_name: String, invited_by: String },
    #[serde(rename = "team_join")]
    TeamJoin { team_id: String, user_name: String },
    #[serde(rename = "team_leave")]
    TeamLeave { team_id: String, user_name: String },
    #[serde(rename = "deadline_reminder")]
    DeadlineReminder { hackathon_id: String, deadline_name: String, hours_left: i32 },
    #[serde(rename = "submission_status")]
    SubmissionStatus { submission_id: String, status: String },
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    state: SharedState,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

pub async fn handle_socket(mut socket: WebSocket, state: SharedState) {
    let mut rx = state.ws_tx.subscribe();

    loop {
        tokio::select! {
            msg = rx.recv() => {
                match msg {
                    Ok(msg) => {
                        if socket.send(Message::Text(msg)).await.is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
                }
            }
        }
    }
}

pub fn broadcast_message(state: &SharedState, msg: WsMessage) {
    if let Ok(json) = serde_json::to_string(&msg) {
        let _ = state.ws_tx.send(json);
    }
}
