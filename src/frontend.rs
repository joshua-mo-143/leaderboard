use axum::{
    response::{Html, IntoResponse, Response},
    Extension,
    http::StatusCode,
    extract::Query
};
use std::sync::Arc;
use tera::{Context, Tera};
use shuttle_persist::PersistInstance;
use serde_json::json;
use serde::Deserialize;

use crate::websocket::load_persist;
use crate::websocket::PlayerScore;

#[derive(Deserialize)]
pub struct Pagination {
	page: Option<usize>
}

pub async fn index(Extension(persist): Extension<PersistInstance>,
Extension(templates): Extension<Arc<Tera>>,
Query(pagination): Query<Pagination>
) -> impl IntoResponse {
	let offset = match pagination.page {
	Some(x) => { (x - 1) * 10 },
	None => 0
	};

    let scores_raw = load_persist(persist);

	let scores: Vec<PlayerScore> = scores_raw.clone().into_iter().skip(offset).take(10).collect();
	
    let pages = { 
	let pages = { scores_raw.len() / 10 } as i32;
	pages + 1 
	};

    let mut ctx = Context::new();
    ctx.insert("scores", &json!(scores));
    ctx.insert("pages", &pages);
    ctx.insert("offset", &offset);
    
    Html(templates.render("index", &ctx).unwrap())
}

pub async fn styles() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(include_str!("../templates/styles.css").to_owned())
        .unwrap()

}

