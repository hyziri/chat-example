use axum::{
    extract::{FromRequestParts, Request},
    http::{HeaderMap, StatusCode, request::Parts},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use tower_sessions::Session;
use uuid::Uuid;

use crate::{error::AppError, model::session::user::SessionUserId};

/// Authenticated user ID stored in request extensions
/// Handlers can extract this to access the current user's ID without re-reading the session
#[derive(Clone, Debug)]
pub struct AuthenticatedUserId(pub Uuid);

/// Implement FromRequestParts so handlers can easily extract the authenticated user ID
/// Usage: `async fn handler(AuthenticatedUserId(user_id): AuthenticatedUserId) -> impl IntoResponse`
impl<S> FromRequestParts<S> for AuthenticatedUserId
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthenticatedUserId>()
            .cloned()
            .ok_or_else(|| {
                AppError::Internal(
                    "Missing authenticated user ID. Is the auth middleware applied?".to_string(),
                )
            })
    }
}

/// Middleware to ensure user is authenticated
pub async fn require_auth(
    session: Session,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Response {
    // Check if user is authenticated
    match SessionUserId::get(&session).await {
        Ok(Some(user_id)) => {
            // User is authenticated, store user_id in request extensions
            // so handlers can access it without re-reading the session
            let mut request = request;
            request
                .extensions_mut()
                .insert(AuthenticatedUserId(user_id));
            next.run(request).await
        }
        Ok(None) => {
            // No user in session - redirect to login
            handle_unauthenticated(&headers)
        }
        Err(_) => {
            // Error reading session - treat as unauthenticated
            handle_unauthenticated(&headers)
        }
    }
}

/// Handle unauthenticated requests based on request type
fn handle_unauthenticated(headers: &HeaderMap) -> Response {
    // Check if this is an HTMX request
    let is_htmx = headers
        .get("HX-Request")
        .and_then(|v| v.to_str().ok())
        .map(|v| v == "true")
        .unwrap_or(false);

    if is_htmx {
        // For HTMX requests, trigger a client-side redirect
        // This maintains SPA-like behavior
        (StatusCode::OK, [("HX-Redirect", "/login")], "").into_response()
    } else {
        // For regular requests, use standard redirect
        Redirect::to("/login").into_response()
    }
}
