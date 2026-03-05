use axum::{
    Router, middleware,
    routing::{get, post},
};
use tower_http::compression::CompressionLayer;

use crate::{
    controller::{home, login, register},
    middleware::auth::require_auth,
    model::app::AppState,
};

pub fn create_router() -> Router<AppState> {
    // Protected routes - require authentication
    let protected = Router::new()
        .route("/", get(home::index))
        .layer(middleware::from_fn(require_auth));

    // Public routes - no authentication required
    let public = Router::new()
        // HTTP pages
        .route("/login", get(login::index))
        .route("/register", get(register::index))
        // HTMX Forms
        .route("/register", post(register::register));

    // Combine routes
    Router::new()
        .merge(protected)
        .merge(public)
        // Add gzip compression for all responses
        .layer(CompressionLayer::new())
}
