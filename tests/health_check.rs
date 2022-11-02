use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;
use zero2prod::app; // for `oneshot` and `ready`

#[tokio::test]
async fn health_check() {
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
}
