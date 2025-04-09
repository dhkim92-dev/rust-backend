use axum_extra::extract::cookie::{Cookie, SameSite};
use shaku::{Interface, Component};
use std::sync::Arc;

use crate::config::ConfigProvider;


#[async_trait::async_trait]
pub trait CookieMaker: Interface {
    
    fn create_cookie(&self, name: String, value: String) -> Cookie<'static>;
}

#[derive(Component)]
#[shaku(interface = CookieMaker)]
pub struct CookieMakerImpl {
    #[shaku(inject)]
    config: Arc<dyn ConfigProvider>,
}

impl CookieMaker for CookieMakerImpl {

    fn create_cookie(&self, name: String, value: String) -> Cookie<'static> {
        let mut cookie = Cookie::new(name, value);
        cookie.set_path("/");
        cookie.set_http_only(self.config.get().is_production());
        cookie.set_secure(self.config.get().is_production());
        cookie.set_same_site(SameSite::Lax);
        cookie.into_owned()
    }
}
