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

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let app = app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .uri("/subscriptions")
                .body(Body::from("name=Le Guin&email=ursula_le_guin@gmail.com"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = app();

    let test_cases = vec![
        ("name=Le Guin", "missing the email"),
        ("email=ursula_le_guin@gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .uri("/subscriptions")
                    .body(Body::from(invalid_body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(
            response.status(),
            StatusCode::UNPROCESSABLE_ENTITY,
            "The API did not fail with 422 Bad Request when the payload was {}.",
            error_message
        );
    }
}
