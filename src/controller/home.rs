use axum::{
    extract::State,
    http::HeaderMap,
    response::{Html, IntoResponse},
};
use tera::Context;

use crate::{error::AppError, model::app::AppState};

/// GET / - Render the homepage
pub async fn index(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    let mut context = Context::new();
    context.insert("current_page", "home");

    // Check if this is an HTMX request
    let is_htmx = headers
        .get("HX-Request")
        .and_then(|v| v.to_str().ok())
        .map(|v| v == "true")
        .unwrap_or(false);

    // Return content-only template for HTMX requests (SPA navigation)
    let template = if is_htmx {
        "index_content.html"
    } else {
        "index.html"
    };

    let html = state
        .templates()
        .render(template, &context)
        .map_err(|e| AppError::Internal(format!("Template error: {}", e)))?;

    Ok(Html(html))
}
