use axum::{
    routing::get,
    Json, Router
};

use std::net::SocketAddr;

mod interfaces;


#[tokio::main]
async fn main() {

    let app = Router::new()
        .nest("/api/v1/auth", interfaces::auth::controller::create_router());


    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind TCP listener");
    println!("Listening on {}", listener.local_addr().unwrap());


    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

