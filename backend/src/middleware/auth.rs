use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::{
    utils::{validate_token, extract_token_from_header, AppError},
    AppState,
};

/// Middleware to authenticate requests using JWT
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    // Extract token from header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                "Missing authorization header".to_string(),
            )
        })?;

    let token = extract_token_from_header(auth_header).map_err(|e| {
        (
            StatusCode::UNAUTHORIZED,
            format!("Invalid authorization header: {}", e),
        )
    })?;

    // Validate token
    let claims = validate_token(&token, &state.config.jwt.secret).map_err(|e| {
        (
            StatusCode::UNAUTHORIZED,
            format!("Invalid token: {}", e),
        )
    })?;

    // Insert user ID into request extensions
    request.extensions_mut().insert(claims.sub);

    Ok(next.run(request).await)
}

/// Extractor for getting the authenticated user ID from request extensions
#[axum::async_trait]
impl<S> axum::extract::FromRequestParts<S> for crate::models::AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let user_id = parts
            .extensions
            .get::<String>()
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    "User ID not found in request".to_string(),
                )
            })?
            .clone();

        Ok(crate::models::AuthUser { user_id })
    }
}
