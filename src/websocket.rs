use serde_json::json;

use crate::{ScoreReceiver, ScoreSender};
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    http::StatusCode,
    response::Response,
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt as _;
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct PlayerScore {
    score: u32,
    name: String,
}

pub async fn socket_handler(
    ws: WebSocketUpgrade,
    Extension(rx): Extension<ScoreReceiver>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, rx))
}

pub async fn handle_socket(mut socket: WebSocket, rx: ScoreReceiver) {
    let mut vec: Vec<PlayerScore> = Vec::with_capacity(10);
    while let Some(message) = rx.lock().await.next().await {
        if vec.len() == 10 {
            vec.pop();
        }
        vec.push(message.clone());
        vec.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
        println!("Received score: {:?}", message);
        let message = Message::Text(json!(message).to_string());
        if socket.send(message).await.is_err() {
            println!("Client error while sending");
            // client disconnected
            return;
        }
    }
}

pub async fn post_score(
    Extension(tx): Extension<ScoreSender>,
    Json(submission): Json<PlayerScore>,
) -> StatusCode {
    tx.lock().await.send(submission).unwrap();

    StatusCode::OK
}
