use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;
use uuid::Uuid;

use crate::{
    models::{User, UserResponse, AuthUser},
    utils::{hash_password, verify_password, generate_token, AppError, Result},
    AppState,
};

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub token: String,
}

/// Register a new user
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>)> {
    // Validate input
    payload.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Check if user already exists
    let existing_user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&payload.email)
    .fetch_optional(&state.db)
    .await?;

    if existing_user.is_some() {
        return Err(AppError::Conflict("Email already registered".to_string()));
    }

    // Hash password
    let password_hash = hash_password(&payload.password)?;

    // Create user
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (email, password_hash, first_name, last_name)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#
    )
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(&payload.first_name)
    .bind(&payload.last_name)
    .fetch_one(&state.db)
    .await?;

    // Generate JWT token
    let token = generate_token(
        user.id,
        &user.email,
        &state.config.jwt.secret,
        state.config.jwt.expiration,
    )?;

    tracing::info!("User registered: {}", user.email);

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            user: user.into(),
            token,
        }),
    ))
}

/// Login user
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>> {
    // Validate input
    payload.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Find user by email
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&payload.email)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::Authentication("Invalid credentials".to_string()))?;

    // Check if user is active
    if !user.is_active {
        return Err(AppError::Authentication("Account is deactivated".to_string()));
    }

    // Verify password
    if !verify_password(&payload.password, &user.password_hash)? {
        return Err(AppError::Authentication("Invalid credentials".to_string()));
    }

    // Update last login time
    sqlx::query(
        "UPDATE users SET last_login_at = NOW() WHERE id = $1"
    )
    .bind(user.id)
    .execute(&state.db)
    .await?;

    // Generate JWT token
    let token = generate_token(
        user.id,
        &user.email,
        &state.config.jwt.secret,
        state.config.jwt.expiration,
    )?;

    tracing::info!("User logged in: {}", user.email);

    Ok(Json(AuthResponse {
        user: user.into(),
        token,
    }))
}

/// Get current user info
pub async fn me(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<UserResponse>> {
    let user_uuid: Uuid = auth_user.user_id.parse()
        .map_err(|_| AppError::Authentication("Invalid user ID".to_string()))?;

    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(user_uuid)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(Json(user.into()))
}
