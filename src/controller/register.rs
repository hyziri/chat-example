use axum::{
    Form,
    extract::State,
    http::HeaderMap,
    response::{Html, IntoResponse},
};
use tera::Context;

use crate::{error::AppError, model::app::AppState};

/// GET /register - Render the registration page
pub async fn index(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    let mut context = Context::new();
    context.insert("current_page", "register");

    // Check if this is an HTMX request
    let is_htmx = headers
        .get("HX-Request")
        .and_then(|v| v.to_str().ok())
        .map(|v| v == "true")
        .unwrap_or(false);

    // Return just the register form for swaps, otherwise full page with form
    let template = if is_htmx {
        "register/index_content.html"
    } else {
        "register/index.html"
    };

    let html = state
        .templates()
        .render(template, &context)
        .map_err(|e| AppError::Internal(format!("Template error: {}", e)))?;

    Ok(Html(html))
}

/// POST /register - Handle user registration
pub async fn register(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    // TODO: Implement user registration logic
    // 1. Validate form data
    // 2. Check if email already exists
    // 3. Store user in database with E2EE fields
    // 4. Generate recovery codes
    // 5. Return success response with recovery codes

    // For now, just return a placeholder
    Ok(Html(
        "<div>Registration endpoint - TODO: Implement</div>".to_string(),
    ))
}
