use axum::{routing::get, routing::post, Form, Router};

#[must_use]
pub fn app() -> Router {
    // build our application with a single route
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
}

#[allow(clippy::unused_async)]
async fn health_check() {}

#[allow(clippy::unused_async)]
async fn subscribe(Form(_input): Form<SubscriptionFormData>) {}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct SubscriptionFormData {
    email: String,
    name: String,
}

impl SubscriptionFormData {
    #[must_use]
    pub fn new(email: &str, name: &str) -> Self {
        SubscriptionFormData {
            email: email.to_string(),
            name: name.to_string(),
        }
    }
}
