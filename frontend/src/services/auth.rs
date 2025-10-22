use serde::{Deserialize, Serialize};
use super::api::{self, ApiError};

/// User data model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_active: bool,
    pub is_email_verified: bool,
    pub created_at: String,
}

impl User {
    pub fn full_name(&self) -> String {
        match (&self.first_name, &self.last_name) {
            (Some(first), Some(last)) => format!("{} {}", first, last),
            (Some(first), None) => first.clone(),
            (None, Some(last)) => last.clone(),
            (None, None) => self.email.clone(),
        }
    }
}

/// Registration request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

/// Login request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Authentication response (for both login and register)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub user: User,
    pub token: String,
}

/// Register a new user
pub async fn register(
    email: String,
    password: String,
    first_name: Option<String>,
    last_name: Option<String>,
) -> Result<AuthResponse, ApiError> {
    let request = RegisterRequest {
        email,
        password,
        first_name,
        last_name,
    };

    let response: AuthResponse = api::post("/api/auth/register", &request).await?;
    
    // Save token to localStorage
    api::set_token(&response.token);
    
    Ok(response)
}

/// Login user
pub async fn login(email: String, password: String) -> Result<AuthResponse, ApiError> {
    let request = LoginRequest { email, password };
    
    let response: AuthResponse = api::post("/api/auth/login", &request).await?;
    
    // Save token to localStorage
    api::set_token(&response.token);
    
    Ok(response)
}

/// Logout user
pub fn logout() {
    api::remove_token();
}

/// Get current user info
pub async fn get_current_user() -> Result<User, ApiError> {
    api::get("/api/auth/me").await
}

/// Check if user is authenticated (has token)
pub fn is_authenticated() -> bool {
    api::get_token().is_some()
}
