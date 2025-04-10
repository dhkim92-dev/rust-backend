use std::collections::HashMap;

use axum_extra::extract::CookieJar;
use derive_builder::Builder;
use serde::{Serialize, Deserialize};
use crate::common::AppError;

#[async_trait::async_trait]
pub trait OAuth2Usecase {

    fn redirect_to_login_page(&self, jar: CookieJar, mode: String) -> (CookieJar, String);

    async fn get_userinfo(&self, 
        jar: CookieJar,
        code: String,
        grant_code: String) -> Result<(CookieJar, OAuth2UserProfile), AppError>;
}

#[derive(Serialize, Deserialize, Clone)]
pub enum OAuth2Provider {
    Github,
    Google,
}

#[derive(Serialize, Deserialize, Clone, Builder)]
pub struct OAuth2Request {
    #[builder(default="OAuth2Provider::Github")]
    pub provider: OAuth2Provider,
    pub authorization_uri: String,
    pub redirect_uri: String,
    pub client_id: String,
    pub scope: String,
    pub response_type: String,
    #[builder(default=None)]
    pub state: Option<String>,
    // pub nounce: Option<String>,
    #[builder(default="true")]
    pub prompt: bool,
    #[builder(default=None)]
    pub full_redirect_uri: Option<String>,
    #[builder(default=HashMap::new())]
    pub additional_params: HashMap<String, String>,
}

impl OAuth2Request {
    pub fn to_redirect_uri(&self) -> Option<String> {
        let request = self.clone();

        if request.response_type.is_empty() || 
            request.client_id.is_empty() ||
            request.redirect_uri.is_empty() ||
            request.authorization_uri.is_empty() {
            return None;
        }

        let mut uri = format!("{}?client_id={}&redirect_uri={}", request.authorization_uri, request.client_id, request.redirect_uri);

        uri.push_str(&format!("&scope={}", request.scope));

        if let Some(ref state) = request.state {
            uri.push_str(&format!("&state={}", state));
        }

        uri.push_str(&format!("&prompt={}", request.prompt));

        for (key, value) in request.additional_params.iter() {
            uri.push_str(&format!("&{}={}", key, value));
        }

        Some(uri)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OAuth2UserProfile {
    pub provider: String,
    pub user_id: String,
    pub email: Option<String>,
    pub access_token: String,
}
