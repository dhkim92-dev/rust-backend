use crate::common::error::error_code::ErrorCode;
use crate::common::wrapper::ReturnValue;
use crate::di::AppContext;
use axum::response::IntoResponse;
use axum::Router;
use axum_extra::extract::cookie::Cookie;
use std::sync::Arc;
use time::*;

async fn handler() -> impl IntoResponse {
    let mut cookie = Cookie::new("test-cookie-for-dohoon-kim.kr", "test");
    cookie.set_expires(OffsetDateTime::now_utc() + Duration::days(1));
    cookie.set_path("/");
    cookie.set_domain("localhost:8080");
    cookie.set_secure(false);
    cookie.set_http_only(true);
    cookie.set_same_site(axum_extra::extract::cookie::SameSite::Lax);
    cookie.set_max_age(Duration::days(1));

    let mut response =
        ReturnValue::new(200, "쿠키가 설정되었습니다.".to_string(), None::<()>).into_response();

    response
        .headers_mut()
        .insert("Set-Cookie", cookie.to_string().parse().unwrap());

    response
}

pub fn router(ctx: Arc<AppContext>) -> Router {
    Router::new()
        .route("/", axum::routing::get(handler))
        .with_state(ctx.clone())
}
