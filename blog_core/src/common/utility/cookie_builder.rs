use crate::config::ConfigProvider;
use axum_extra::extract::cookie::Cookie;
use shaku::{Component, Interface};
use std::sync::Arc;

pub trait CookieBuilder: Interface {
    fn build(&self, name: &str, value: &str, max_age: Option<i64>) -> Cookie;
}

#[derive(Component)]
#[shaku(interface = CookieBuilder)]
pub struct CookieBuilderImpl {
    #[shaku(inject)]
    config_provider: Arc<dyn ConfigProvider>,
}

impl CookieBuilder for CookieBuilderImpl {
    fn build(&self, name: &str, value: &str, max_age: Option<i64>) -> Cookie {
        let cfg = self.config_provider.get();
        let mut cookie = Cookie::new(name.to_string(), value.to_string());
        cookie.set_path("/");
        cookie.set_http_only(true);
        cookie.set_secure(cfg.is_production());
        cookie
    }
}
