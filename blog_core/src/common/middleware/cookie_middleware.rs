use axum::{body::Body, http::Request, middleware::Next, response::Response};
use axum_extra::extract::cookie::CookieJar;

use crate::common::error_code::ErrorCode;

pub async fn cookie_middleware(mut req: Request<Body>, next: Next) -> Result<Response, ErrorCode> {
    let cookie_jar = CookieJar::from_headers(&req.headers());
    req.extensions_mut().insert(cookie_jar.clone());
    let mut response = next.run(req).await;

    if let Some(cookie_jar) = response.extensions_mut().remove::<CookieJar>() {
        for cookie in cookie_jar.iter() {
            response.headers_mut().append(
                axum::http::header::SET_COOKIE,
                cookie.to_string().parse().unwrap(),
            );
        }
    }

    Ok(response)
}
