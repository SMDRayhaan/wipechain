use axum::{routing::{get, post}, Router};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use crate::service::routes::{
    health::health_handler,
    disks::{disks_handler, preview_wipe_handler, wipe_handler},
};

pub async fn start_server() {
    let app = Router::new()
    .route("/health", get(health_handler))
    .route("/disks", get(disks_handler))
    .route("/preview/:id", get(preview_wipe_handler))
    .route("/wipe/:id", post(wipe_handler))
    .layer(CorsLayer::permissive());

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind");

    println!("server running on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("server failed");
}