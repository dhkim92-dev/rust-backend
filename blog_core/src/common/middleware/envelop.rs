use crate::common::{
    error::error_code::ErrorCode,
    wrapper::{ApiResponse, ReturnValue},
};
use axum::{
    body::Body, extract::Request, http::StatusCode, middleware::Next, response::IntoResponse,
    response::Response,
};
use serde_json::Value;
/* 
pub async fn envelop_pattern_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, impl IntoResponse> {
    let response: Response<Body> = next.run(req).await;

    if response.status().is_client_error()
        || response.status().is_server_error()
    {
        return Err(response);
    }

    if response.status() == StatusCode::NO_CONTENT {
        return Ok(response);
    }

    let status = response.status();
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .map_err(|_| {
            Err(
            ErrorCode::with_message(
                ErrorCode::InternalServerError,
                "응답을 처리하는 중 오류가 발생했습니다.",
            ))
        });

    let payload = serde_json::from_slice::<ReturnValue<Value>>(&body_bytes).map_err(|_| {
        Err(ErrorCode::with_message(
            ErrorCode::InternalServerError,
            "응답을 처리하는 중 오류가 발생했습니다.",
        ))
    });

    let enveloped = ApiResponse::new(payload.status, payload.message, Some(payload.data));
    Ok(enveloped.into_response()) */

    /* return Ok(Response::builder()
    .status(status)
    .header("Content-Type", "application/json; charset=utf-8")
    .body(Body::from(serde_json::to_string(&enveloped).unwrap()))
    .map_err(|_| {
        ErrorCode::with_message(
            ErrorCode::InternalServerError,
            "응답을 처리하는 중 오류가 발생했습니다.",
        )
    })?); */
// }
