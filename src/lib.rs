use crate::core::router::initialize_router;
use crate::utils::load_config::AppConfig;
use axum::Router;
use std::sync::Arc;

pub mod core;
pub mod utils;

/// Shared application state.
#[derive(Clone)]
pub struct AppState {
    /// Global application configuration.
    pub config: Arc<AppConfig>,
}

/// Factory function to build the main application router.
pub fn create_app(state: AppState) -> Router {
    let state_arc = Arc::new(state);
    initialize_router(state_arc)
}
