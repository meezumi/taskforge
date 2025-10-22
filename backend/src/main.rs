use axum::{
    routing::{get, post},
    Router,
    response::Json,
    middleware as axum_middleware,
};
use serde_json::{json, Value};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use sqlx::postgres::PgPoolOptions;

mod api;
mod config;
mod middleware;
mod models;
mod services;
mod utils;

use config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub config: Arc<Config>,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "taskforge_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    tracing::info!("🚀 TaskForge API starting...");

    // Load configuration
    let config = Arc::new(Config::from_env().expect("Failed to load configuration"));
    
    // Create database connection pool
    tracing::info!("📊 Connecting to database...");
    let db = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.url)
        .await
        .expect("Failed to connect to database");
    
    tracing::info!("✅ Database connected");

    // Create app state
    let state = AppState {
        db: db.clone(),
        config: config.clone(),
    };

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build application routes
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        // Public auth routes
        .route("/api/auth/register", post(api::register))
        .route("/api/auth/login", post(api::login))
        // Protected auth routes
        .route("/api/auth/me", get(api::me))
        // Protected organization routes
        .route("/api/organizations", post(api::create_organization).get(api::get_my_organizations))
        .route("/api/organizations/:org_id", get(api::get_organization))
        .route("/api/organizations/:org_id/members", get(api::get_organization_members))
        .route_layer(axum_middleware::from_fn_with_state(
            state.clone(),
            crate::middleware::auth_middleware,
        ))
        .with_state(state)
        .layer(cors)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    // Get server address from config
    let addr: SocketAddr = format!("{}:{}", config.server.host, config.server.port)
        .parse()
        .expect("Failed to parse address");

    tracing::info!("🎯 Server listening on http://{}", addr);

    // Start server
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

async fn root() -> Json<Value> {
    Json(json!({
        "name": "TaskForge API",
        "version": "0.1.0",
        "status": "running"
    }))
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
