use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::window;

/// API base URL
pub const API_BASE_URL: &str = "http://localhost:3000";

/// API error type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub message: String,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ApiError {}

impl From<gloo_net::Error> for ApiError {
    fn from(err: gloo_net::Error) -> Self {
        ApiError {
            message: format!("Network error: {}", err),
        }
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError {
            message: format!("JSON error: {}", err),
        }
    }
}

/// Get JWT token from localStorage
pub fn get_token() -> Option<String> {
    window()?
        .local_storage()
        .ok()??
        .get_item("auth_token")
        .ok()?
}

/// Save JWT token to localStorage
pub fn set_token(token: &str) {
    if let Some(storage) = window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
    {
        let _ = storage.set_item("auth_token", token);
    }
}

/// Remove JWT token from localStorage
pub fn remove_token() {
    if let Some(storage) = window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
    {
        let _ = storage.remove_item("auth_token");
    }
}

/// Make a GET request
pub async fn get<T: for<'de> Deserialize<'de>>(endpoint: &str) -> Result<T, ApiError> {
    let url = format!("{}{}", API_BASE_URL, endpoint);
    let mut request = Request::get(&url);

    // Add Authorization header if token exists
    if let Some(token) = get_token() {
        request = request.header("Authorization", &format!("Bearer {}", token));
    }

    let response = request.send().await?;
    
    if response.ok() {
        let data = response.json::<T>().await?;
        Ok(data)
    } else {
        let error_text = response.text().await.unwrap_or_else(|_| {
            format!("HTTP error: {}", response.status())
        });
        Err(ApiError {
            message: error_text,
        })
    }
}

/// Make a POST request
pub async fn post<T: Serialize, R: for<'de> Deserialize<'de>>(
    endpoint: &str,
    body: &T,
) -> Result<R, ApiError> {
    let url = format!("{}{}", API_BASE_URL, endpoint);
    let body_json = serde_json::to_string(body)?;
    
    let mut request = Request::post(&url)
        .header("Content-Type", "application/json");

    // Add Authorization header if token exists
    if let Some(token) = get_token() {
        request = request.header("Authorization", &format!("Bearer {}", token));
    }

    let response = request.body(body_json)?.send().await?;
    
    if response.ok() {
        let data = response.json::<R>().await?;
        Ok(data)
    } else {
        let error_text = response.text().await.unwrap_or_else(|_| {
            format!("HTTP error: {}", response.status())
        });
        Err(ApiError {
            message: error_text,
        })
    }
}
