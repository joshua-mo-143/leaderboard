use tokio::sync::watch::{channel, Sender};
use tokio::sync::Mutex;
use std::sync::Arc;
use axum::{
extract::ws::{WebSocketUpgrade, WebSocket, Message},	
    http::StatusCode,
    routing::{get, post},
    Extension, Json, Router,
    response::{Response},
};
use serde::Deserialize;
use tokio_stream::{StreamExt as _, wrappers::WatchStream};

type ScoreReceiver = Arc<Mutex<WatchStream<PlayerScore>>>;
type ScoreSender = Arc<Mutex<Sender<PlayerScore>>>;
async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
	let score = PlayerScore::default();
    let (tx, rx) = channel(score);
	let rx = Arc::new(Mutex::new(WatchStream::from_changes(rx)));
	let tx = Arc::new(Mutex::new(tx));

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/post_score", post(post_score))
	.route("/ws", get(handler))
	.layer(Extension(tx))
	.layer(Extension(rx));


    Ok(router.into())
}

#[derive(Deserialize, Default, Clone)]
struct PlayerScore {
    score: u32,
    name: String,
}

#[axum::debug_handler]
async fn post_score(
	Extension(tx): Extension<ScoreSender>,
    Json(submission): Json<PlayerScore>,
) -> StatusCode {
	tx.lock().await.send(submission).unwrap();

    StatusCode::OK
}

async fn handler(ws: WebSocketUpgrade, Extension(rx): Extension<ScoreReceiver>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, rx))
}

async fn handle_socket(mut socket: WebSocket, rx: ScoreReceiver) {
 	while let Some(_message) = rx.lock().await.next().await {
	let message = Message::Text("Hello world".to_string());
        if socket.send(message).await.is_err() {
            // client disconnected
            return;
        }
    }
}
