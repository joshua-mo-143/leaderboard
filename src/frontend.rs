use axum::{
    response::{Html, IntoResponse},
    Extension,
};
use std::sync::Arc;
use tera::{Context, Tera};

pub async fn index(Extension(templates): Extension<Arc<Tera>>) -> impl IntoResponse {
    Html(templates.render("index", &Context::new()).unwrap())
}
