//! # Chat WebSocket Server Binary
//!
//! The entry point for the WebSocket server. Handles:
//! - Environment/Config loading and validation.
//! - Logging initialization.
//! - Server binding and serving.

use chat_web_socket_server::utils::load_config::load_config;
use chat_web_socket_server::utils::load_env::load_env;
use chat_web_socket_server::{AppState, create_app};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{error, info};
use tracing_subscriber::fmt::time::SystemTime;

/// Initializes the global tracing subscriber with JSON formatting.
fn initialize_logging() {
    tracing_subscriber::fmt()
        .json()
        .with_timer(SystemTime)
        .with_level(true)
        .init();
}

#[tokio::main]
async fn main() {
    load_env();
    initialize_logging();

    let app_config = match load_config() {
        Ok(config) => {
            if let Err(e) = config.validate() {
                error!("SERVER START-UP ERROR: CONFIG VALIDATION FAILED, {}", e);
                std::process::exit(1);
            }
            config
        }
        Err(e) => {
            error!(
                "SERVER START-UP ERROR: FAILED TO LOAD CONFIGURATIONS, {}",
                e
            );
            std::process::exit(1);
        }
    };

    let state = AppState {
        config: Arc::new(app_config),
    };

    let app = create_app(state.clone());

    let host = state
        .config
        .server
        .as_ref()
        .map(|s| s.host.as_str())
        .unwrap_or("127.0.0.1");
    let port = state.config.server.as_ref().map(|s| s.port).unwrap_or(8001);

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid server address");

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => {
            println!(
                "
                .................................................
                App: {}
                Environment: {}
                Status: Running
                .................................................

                Server running on http://{}
                ",
                state.config.app.name,
                state.config.app.environment.as_deref().unwrap_or("unknown"),
                addr
            );
            listener
        }
        Err(e) => {
            error!("SERVER INITIALIZATION ERROR: {}!", e);
            std::process::exit(1);
        }
    };

    match axum::serve(listener, app).await {
        Ok(_) => info!("Graceful server shutdown!"),
        Err(e) => error!("SERVER SHUTDOWN ERROR: {}!", e),
    }
}
