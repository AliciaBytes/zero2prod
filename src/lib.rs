use axum::{routing::get, Router};

#[must_use]
pub fn app() -> Router {
    // build our application with a single route
    Router::new().route("/health_check", get(health_check))
}

#[allow(clippy::unused_async)]
async fn health_check() {}
