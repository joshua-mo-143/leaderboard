use axum::{
    routing::{get, post},
    Extension, Router,
};
use std::path::PathBuf;
use std::sync::Arc;
use tera::Tera;
use tokio::sync::watch::{channel, Sender};
use tokio::sync::Mutex;
use tokio_stream::wrappers::WatchStream;

use frontend::index;
use websocket::{post_score, socket_handler, PlayerScore};
mod frontend;
mod websocket;

pub type ScoreReceiver = Arc<Mutex<WatchStream<PlayerScore>>>;
pub type ScoreSender = Arc<Mutex<Sender<PlayerScore>>>;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder(folder = "templates")] templates: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    let tera = init_tera(templates);

    let score = PlayerScore::default();
    let (tx, rx) = channel(score);
    let rx = Arc::new(Mutex::new(WatchStream::from_changes(rx)));
    let tx = Arc::new(Mutex::new(tx));

    let router = Router::new()
        .route("/", get(index))
        .route("/submit", post(post_score))
        .route("/ws", get(socket_handler))
        .layer(Extension(tx))
        .layer(Extension(rx))
        .layer(Extension(Arc::new(tera)));

    Ok(router.into())
}

pub fn init_tera(templates: PathBuf) -> Tera {
    let tera = format!("{}/**/*", templates.display());

    let mut tera = Tera::new(&tera).unwrap();
    tera.add_template_files(vec![
        (templates.join("base.html"), Some("base")),
        (templates.join("index.html"), Some("index")),
    ])
    .unwrap();

    tera
}
