pub mod error;
pub mod auth;

pub use error::{AppError, Result};
pub use auth::{hash_password, verify_password, generate_token, validate_token, extract_token_from_header, Claims};
