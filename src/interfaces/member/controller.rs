use axum::{
    routing::post,
    Router,
    Json
};
use tracing::info;

use crate::interfaces::auth::dto::{
    LoginRequest,
    LoginResponse
};


async fn get_members() -> Json<CursorList<>> {
    info!("Login request: {:?}", req);

    Json(LoginResponse {
        typ: "Bearer".to_string(),
        access_token: "access_token".to_string(),
        refresh_token: "refresh_token".to_string(),
    })
}

pub fn create_router() -> Router {
    info!("Creating auth router");
    Router::new()
        .route("/", post(login))
}
