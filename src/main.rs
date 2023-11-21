use std::net::TcpListener;

use rust_todo::app;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app().into_make_service())
        .await
        .unwrap();
}
