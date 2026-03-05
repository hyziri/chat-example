use axum::{
    Router,
    routing::{get, post},
};
use tower_http::compression::CompressionLayer;

use crate::{
    controller::{home, login, register},
    model::app::AppState,
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        // HTML pages
        .route("/", get(home::index))
        .route("/login", get(login::index))
        .route("/register", get(register::index))
        // HTMX form submission routes
        .route("/register", post(register::register))
        // Add gzip compression for all responses
        .layer(CompressionLayer::new())
}
