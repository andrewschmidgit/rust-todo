use askama::Template;
use axum::{http::StatusCode, routing::get, Router, response::{IntoResponse, Html}};

async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub fn app() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/", get(todos_index))
        .layer(tower_livereload::LiveReloadLayer::new())
}

#[derive(Template)]
#[template(path = "todos.html")]
struct TodosTemplate {
    name: String,
}

async fn todos_index() -> impl IntoResponse {
    let t = TodosTemplate {
        name: "Tom".into()
    };
    let html = t.render().unwrap();
    Html(html).into_response()
}
