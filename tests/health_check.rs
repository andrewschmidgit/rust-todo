use axum::{
    body::{Body, HttpBody},
    http::{Request, StatusCode},
};
use rust_todo::app;
use tower::ServiceExt;

#[tokio::test]
async fn health_check_returns_200() {
    let app = app();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health_check")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert!(response.into_body().data().await.is_none());
}
