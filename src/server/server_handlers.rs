use askama_axum::IntoResponse;
use axum::response::Html;

use askama::Template;

#[derive(Template)]
#[template(path = "server/index.html")]
pub struct IndexTemplate {}

pub async fn index() -> impl IntoResponse {
    let template = IndexTemplate {};
    return Html(template.render().unwrap()).into_response();
}
