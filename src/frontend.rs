use axum::{
    response::{Html, IntoResponse},
    Extension,
};
use std::sync::Arc;
use tera::{Context, Tera};

pub async fn index(Extension(templates): Extension<Arc<Tera>>,
	Extension(domain): Extension<String>) -> impl IntoResponse {
	
	let mut ctx = Context::new();

	ctx.insert("domain", &domain);

    Html(templates.render("index", &ctx).unwrap())
}
