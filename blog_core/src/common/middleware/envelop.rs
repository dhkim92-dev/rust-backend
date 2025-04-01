use axum::{body::Body, extract::Request, http::StatusCode, middleware::Next, response::Response, Error};
use serde_json::Value;
use tracing::{field::debug, info};
use crate::common::{error::error_code::ErrorCode, wrapper::{ApiResponse, ReturnValue}};


pub async fn envelop_pattern_middleware(
    req: Request,
    next: Next
) -> Result <Response<Body>, ErrorCode> {

    let response = next.run(req).await;
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await
        .map_err(|_| ErrorCode::INTERNAL_SERVER_ERROR)?;
    let body: Value = serde_json::from_slice(&body_bytes)
        .map_err(|_| ErrorCode::INTERNAL_SERVER_ERROR)?;

    if let Ok(return_value) = serde_json::from_value::<ReturnValue<Value>>(body.clone()) {
        let status = return_value.status;
        let message = return_value.message;
        let data = return_value.data;
        let enveloped = ApiResponse::new(status, message, Some(data));
        info!("enveloped response: {:?}", enveloped);

        return Ok(Response::builder()
            .status(status)
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_string(&enveloped).unwrap()))
            .unwrap());
    } 

    Ok(Response::builder()
        .status(200)
        .body(Body::from(body_bytes))
        .unwrap())
} 
