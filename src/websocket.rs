use serde_json::json;
use std::{convert::Infallible};
use crate::{ScoreSender};
use axum::{
    http::StatusCode,
    response::{Sse, sse::{Event}, IntoResponse},
    Extension, Json,
};
use tokio_stream::wrappers::BroadcastStream;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt as _;
use futures_util::stream::{Stream};
use shuttle_persist::PersistInstance;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct PlayerScore {
    score: u32,
    name: String,
}

pub async fn get_scores(
    Extension(persist): Extension<PersistInstance>,	
) -> impl IntoResponse {
    let scores = load_persist(persist);

    (StatusCode::OK, Json(scores))
}

pub async fn post_score(
    Extension(tx): Extension<ScoreSender>,
    Extension(persist): Extension<PersistInstance>,	
    Json(submission): Json<PlayerScore>,
) -> StatusCode {
    let scores = persist_score(persist, submission).unwrap();	

    tx.send(scores).unwrap();
    StatusCode::OK
}

pub async fn handle_stream(
    Extension(tx): Extension<ScoreSender>,	
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
	println!("A new user is trying to connect!");
	let rx = tx.subscribe();

	let stream = BroadcastStream::new(rx);

	    Sse::new(
        stream
            .map(|msg| {
                let msg = msg.unwrap();
		let json = json!(msg).to_string();
                Event::default().data(json)
            })
            .map(Ok),
    )
    .keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(600))
            .text("keep-alive-text")
)

}

fn persist_score(persist: PersistInstance, submission: PlayerScore) -> Result<Vec<PlayerScore>, String> {
	let mut scores = if persist.load::<Vec<PlayerScore>>("scores").is_ok() {
persist.load::<Vec<PlayerScore>>("scores").unwrap()
		} else {
		Vec::new()
};

        scores.push(submission.clone());
        scores.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
	
	if let Err(e) = persist.save::<Vec<PlayerScore>>("scores", scores.to_owned()) {
			println!("Something went wrong with saving scores to file: {e}");
		return Err(format!("Error: {e}"))	
	}
	Ok(scores)
}

pub fn load_persist(persist: PersistInstance) -> Vec<PlayerScore> {

 if persist.load::<Vec<PlayerScore>>("scores").is_ok() {
persist.load::<Vec<PlayerScore>>("scores").unwrap()
		} else {
		Vec::new()
}
}
