use axum::{
    response::{Html, IntoResponse, Response},
    Extension,
    http::StatusCode
};
use std::sync::Arc;
use tera::{Context, Tera};

pub async fn index(Extension(templates): Extension<Arc<Tera>>) -> impl IntoResponse {
    Html(templates.render("index", &Context::new()).unwrap())
}

pub async fn styles() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(include_str!("../templates/styles.css").to_owned())
        .unwrap()

}

