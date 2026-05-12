mod birdeye;
mod routes;
mod types;
mod utils;

use axum::{
    routing::{get,post},
    Router,
};

use tower_http::cors::CorsLayer;

use crate::{
    birdeye::client::BirdeyeClient,
    routes::{analyze::analyze_token, health::health},
    types::app::AppState,
};

#[tokio::main]
async fn main() {
    let app_state = AppState {
        birdeye_client: BirdeyeClient::new(),
    }; 

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/analyze-token", post(analyze_token))
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .expect("failed to bind TCP listener");

    println!("Backend listening on http://localhost:3001");

    axum::serve(listener, app)
        .await
        .expect("server failed");
}
