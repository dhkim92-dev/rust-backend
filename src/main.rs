use axum::{
    routing::get,
    Json, Router
};

use std::net::SocketAddr;

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(get_hello));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind TCP listener");
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

async fn get_hello() -> Json<String> {
    Json("Hello, World!".to_string())
}
