use leptos::*;
use crate::services::auth::{self, User};

/// Auth context to manage authentication state globally
#[derive(Clone, Copy, Debug)]
pub struct AuthContext {
    pub user: RwSignal<Option<User>>,
    pub is_loading: RwSignal<bool>,
    pub error: RwSignal<Option<String>>,
}

impl AuthContext {
    pub fn new() -> Self {
        Self {
            user: create_rw_signal(None),
            is_loading: create_rw_signal(false),
            error: create_rw_signal(None),
        }
    }

    /// Check if user is authenticated
    pub fn is_authenticated(self) -> bool {
        self.user.get().is_some()
    }

    /// Load current user from token
    pub async fn load_user(self) {
        // Only load if we have a token
        if !auth::is_authenticated() {
            return;
        }

        self.is_loading.set(true);
        self.error.set(None);

        match auth::get_current_user().await {
            Ok(user) => {
                self.user.set(Some(user));
                self.error.set(None);
            }
            Err(e) => {
                self.error.set(Some(e.message));
                self.user.set(None);
                auth::logout(); // Clear invalid token
            }
        }

        self.is_loading.set(false);
    }

    /// Login user
    pub async fn login(self, email: String, password: String) -> Result<(), String> {
        self.is_loading.set(true);
        self.error.set(None);

        let result = auth::login(email, password).await;

        match result {
            Ok(response) => {
                self.user.set(Some(response.user));
                self.error.set(None);
                self.is_loading.set(false);
                Ok(())
            }
            Err(e) => {
                self.error.set(Some(e.message.clone()));
                self.is_loading.set(false);
                Err(e.message)
            }
        }
    }

    /// Register user
    pub async fn register(
        self,
        email: String,
        password: String,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<(), String> {
        self.is_loading.set(true);
        self.error.set(None);

        let result = auth::register(email, password, first_name, last_name).await;

        match result {
            Ok(response) => {
                self.user.set(Some(response.user));
                self.error.set(None);
                self.is_loading.set(false);
                Ok(())
            }
            Err(e) => {
                self.error.set(Some(e.message.clone()));
                self.is_loading.set(false);
                Err(e.message)
            }
        }
    }

    /// Logout user
    pub fn logout(self) {
        auth::logout();
        self.user.set(None);
        self.error.set(None);
    }
}

/// Provide auth context to the app
pub fn provide_auth_context() -> AuthContext {
    let auth_context = AuthContext::new();
    provide_context(auth_context);
    auth_context
}

/// Use auth context in components
pub fn use_auth_context() -> AuthContext {
    use_context::<AuthContext>().expect("AuthContext not provided")
}
