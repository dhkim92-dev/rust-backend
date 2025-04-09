// use std::sync::Arc;

use axum_extra::extract::CookieJar;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use crate::application::oauth2::usecases::OAuth2Usecase;
use crate::common::error_code::ErrorCode;
use crate::common::{AppError, CookieMaker};
use crate::config::OAuth2ConfigProvider;
use crate::application::oauth2::{generate_rand,OAuth2Provider, OAuth2UserProfile, OAuth2Request, OAuth2RequestBuilder, OAUTH2_AUTHORIZATION_REQUEST_COOKIE_NAME, OAUTH2_MODE_COOKIE_NAME, OAUTH2_REDIRECT_URI_COOKIE_NAME};

#[derive(serde::Deserialize)]
pub struct GithubAccessTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct GithubUserProfile {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
}

impl Into<OAuth2UserProfile> for GithubUserProfile {
    fn into(self) -> OAuth2UserProfile {
        OAuth2UserProfile {
            provider: "GITHUB".to_string(),
            user_id: self.id.to_string(),
            email: None,
            access_token: "".to_string(),
        }
    }
}

pub struct GithubOAuth2UsecaseImpl<'a> {
    config: &'a dyn OAuth2ConfigProvider,
    cookie_maker: &'a dyn CookieMaker,
}

impl <'a> GithubOAuth2UsecaseImpl<'a> {
    pub fn new(config: &'a dyn OAuth2ConfigProvider, 
        cookie_maker: &'a dyn CookieMaker) -> Self {
        GithubOAuth2UsecaseImpl {
            config,
            cookie_maker
        }
    }

    async fn get_user_profile(&self, access_token: String) ->
        Result<OAuth2UserProfile, AppError> {

        let uri = "https://api.github.com/user";
        let http_client = reqwest::Client::new();
        http_client.get(uri)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Accept", "application/json")
            .header("X-Github-Api-Version", "2022-11-28")
            .send()
            .await
            .map_err(|_| AppError::from(ErrorCode::FailedToGetUserProfile))?
            .json::<GithubUserProfile>()
            .await
            .map_err(|_| AppError::from(ErrorCode::FailedToDeserializeUserProfile))
            .map(|profile| {
                let mut profile: OAuth2UserProfile = profile.into();
                profile.access_token = access_token;
                profile
            })
    }

    async fn request_access_token(&self, grant_code: String) -> Result<String, AppError> {
        let uri = "https://github.com/login/oauth/access_token";
        let http_client = reqwest::Client::new();
        let response = http_client.post(uri)
            .header("Accept", "application/json")
            .header("User-Agent", "reqwest")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(
                format!(
                    "client_id={}&client_secret={}&code={}",
                    self.config.github_client_id(),
                    self.config.github_client_secret(),
                    grant_code,
                )
            )
            .send()
            .await
            .map_err(|_| AppError::with_message(ErrorCode::InternalServerError, "OAuth2 제공자와 통신에서 오류가 발생했습니다."))?;

        let token_response: GithubAccessTokenResponse = response
            .json()
            .await
            .map_err(|_| AppError::with_message(ErrorCode::InternalServerError, "OAuth2 제공자와 통신메시지 해석에 오류가 발생했습니다."))?;

        Ok(token_response.access_token)
    }
}

#[async_trait::async_trait]
impl <'a> OAuth2Usecase for GithubOAuth2UsecaseImpl <'a>{

    fn redirect_to_login_page(&self, jar: CookieJar) -> (CookieJar, String) {

        let mut request = OAuth2RequestBuilder::default()
            .provider(OAuth2Provider::Github)
            .authorization_uri(self.config.github_login_url())
            .redirect_uri(self.config.github_code_redirect_uri())
            .client_id(self.config.github_client_id())
            .scope(self.config.github_scopes())
            .response_type("code".to_owned())
            .state(Some(generate_rand(32)))
            .build()
            .expect("Failed to build OAuth2Request");

        let uri = request.to_redirect_uri().unwrap();
        request.full_redirect_uri = Some(uri.clone());
        let serialized_request = serde_json::to_string(&request).unwrap();
        let encoded_request = URL_SAFE.encode(serialized_request.as_bytes());

        let jar = jar.remove(OAUTH2_AUTHORIZATION_REQUEST_COOKIE_NAME);
        let jar = jar.remove(OAUTH2_REDIRECT_URI_COOKIE_NAME);
        let jar = jar.remove(OAUTH2_MODE_COOKIE_NAME);

        let request_cookie = self.cookie_maker
            .create_cookie(
                OAUTH2_AUTHORIZATION_REQUEST_COOKIE_NAME.to_string(), 
                encoded_request.to_string()
            );
        let redirect_uri_cookie = self.cookie_maker
            .create_cookie(
                OAUTH2_REDIRECT_URI_COOKIE_NAME.to_string(), 
                uri.clone()
            );
        let mode_cookie = self.cookie_maker
            .create_cookie(
                OAUTH2_MODE_COOKIE_NAME.to_string(), 
                "sign-in".to_string()
            );


        let jar = jar.add(request_cookie);
        let jar = jar.add(redirect_uri_cookie);
        let jar = jar.add(mode_cookie);

        (jar, uri)
    }

    async fn get_userinfo(&self, 
        jar: CookieJar, 
        state: String,
        grant_code: String
    ) -> Result<OAuth2UserProfile, AppError> {

        let request_cookie = jar.get(OAUTH2_AUTHORIZATION_REQUEST_COOKIE_NAME)
            .ok_or_else(|| AppError::from(ErrorCode::BadRequest))?;
        let request_cookie = URL_SAFE.decode(request_cookie.value())
            .map_err(|_| AppError::from(ErrorCode::BadRequest))?;
        let request_cookie = serde_json::from_slice::<OAuth2Request>(&request_cookie)
            .map_err(|_| AppError::from(ErrorCode::BadRequest))?;
        let redirect_cookie = jar.get(OAUTH2_REDIRECT_URI_COOKIE_NAME)
            .ok_or_else(|| AppError::from(ErrorCode::BadRequest))?;

        if request_cookie.state != Some(state) {
            return Err(AppError::with_message(
                ErrorCode::Unauthorized, 
                "변조된 OAuth2 요청입니다."
            ));
        }

        let access_token = self.request_access_token(grant_code).await?;
        let user_info = self.get_user_profile(access_token).await?;

        Err(AppError::from(ErrorCode::InternalServerError))
    }
}


