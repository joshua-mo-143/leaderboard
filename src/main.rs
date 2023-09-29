use axum::{
    routing::{get, post},
    Extension, Router,
};
use std::sync::Arc;
use tera::Tera;
use tokio::sync::broadcast::{channel, Sender};

use tokio_stream::wrappers::BroadcastStream;
use frontend::{index, styles};
use websocket::{post_score, handle_stream, PlayerScore, get_scores};
mod frontend;
mod websocket;

pub type ScoreReceiver = BroadcastStream<Vec<PlayerScore>>;
pub type ScoreSender = Sender<Vec<PlayerScore>>;

#[shuttle_runtime::main]
async fn axum(
	#[shuttle_persist::Persist] persist: shuttle_persist::PersistInstance,
	#[shuttle_secrets::Secrets] secrets: shuttle_secrets::SecretStore
) -> shuttle_axum::ShuttleAxum {
    let tera = init_tera();
    let (tx, _rx) = channel::<Vec<PlayerScore>>(1000);
    let domain = secrets.get("DOMAIN_URL").unwrap();

    let router = Router::new()
        .route("/", get(index))
        .route("/styles.css", get(styles))
        .route("/submit", post(post_score))
       	.route("/ws", get(handle_stream))
	.route("/scores", get(get_scores))
        .layer(Extension(tx))
	.layer(Extension(persist))
        .layer(Extension(Arc::new(tera))) 
	.layer(Extension(domain));

    Ok(router.into())
}

pub fn init_tera() -> Tera {
    let tera = "templates/*".to_string();

    let mut tera = Tera::new(&tera).unwrap();
    tera.add_template_files(vec![
        (("templates/base.html"), Some("base")),
        (("templates/index.html"), Some("index")),
    ])
    .unwrap();

    tera
}
