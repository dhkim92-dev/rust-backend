/* use axum::{
    extract::Request, 
    middleware::Next, 
    response::{IntoResponse, Json, Response},
    http::StatusCode
};
use serde::Serialize;

use crate::common::wrapper::ApiResponse;


pub async fn error_handling_middleware<B>(
    req: Request<B>,
    next: Next
) -> Result<Response, Response> {
    match next.run(req).await.into_response().into_result() {
        Ok(response) => Ok(Json(response)),
        Err(err) => {
            let status = err.status();
            let message = err.to_string();
            let response = ApiResponse {
                timestamp: chrono::Utc::now().naive_utc(),
                status: status.as_u16() as i32,
                data: None,
                code: "INTERNAL_SERVER_ERROR".to_string(),
                message: message,
            };
            Err((status, Json(response)).into_response())
        }
    }
} */
