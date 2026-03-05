use axum::{
    extract::State,
    http::HeaderMap,
    response::{Html, IntoResponse},
};
use tera::Context;

use crate::{error::AppError, model::app::AppState};

/// GET /login - Render the login page
pub async fn index(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    let mut context = Context::new();
    context.insert("current_page", "login");

    // Check if this is an HTMX request
    let is_htmx = headers
        .get("HX-Request")
        .and_then(|v| v.to_str().ok())
        .map(|v| v == "true")
        .unwrap_or(false);

    // Return just the login form for swaps, otherwise full page
    let template = if is_htmx {
        "login/index_content.html"
    } else {
        "login/index.html"
    };

    let html = state
        .templates()
        .render(template, &context)
        .map_err(|e| AppError::Internal(format!("Template error: {}", e)))?;

    Ok(Html(html))
}
