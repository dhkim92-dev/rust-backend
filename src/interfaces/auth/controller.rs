use axum::{
    routing::post,
    Router,
    Json
};

use crate::interfaces::auth::dto::{
    LoginRequest,
    LoginResponse
};


async fn login(Json(req): Json<LoginRequest>) -> Json<LoginResponse> {
    println!("Login request: {:?}", req);

    Json(LoginResponse {
        typ: "Bearer".to_string(),
        access_token: "access_token".to_string(),
        refresh_token: "refresh_token".to_string(),
    })
}

pub fn create_router() -> Router {
    println!("Creating auth router");
    Router::new()
        .route("/", post(login))
}
