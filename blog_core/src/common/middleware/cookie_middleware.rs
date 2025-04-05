use crate::common::error_code::ErrorCode;
use axum::{body::Body, http::Request, middleware::Next, response::Response};
use axum_extra::extract::cookie::CookieJar;
use std::sync::Arc;
use tracing::debug;

pub async fn cookie_middleware(mut req: Request<Body>, next: Next) -> Result<Response, ErrorCode> {
    debug!("cookie write middleware");
    let cookie_jar = Arc::new(CookieJar::from_headers(&req.headers()));
    debug!("생성된 쿠키 주소값: {:p}", &cookie_jar);
    req.extensions_mut().insert(cookie_jar.clone());
    let response = next.run(req).await;

    for cookie in cookie_jar.as_ref().iter() {
        debug!("쿠키: {:?}", cookie);
    }

    Ok(response)
}
