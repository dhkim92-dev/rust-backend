use crate::common::AppError;
use crate::common::error_code::ErrorCode;


#[derive(Clone, Eq, PartialEq)]
pub struct OAuth2RedirectURI {
    base_url: String,
    redirect_uri: String,
    client_id: String,
    response_type: Option<String>,
    scope: Option<String>,
    nounce: Option<String>,
    state: Option<String>,
    client_secret: Option<String>,
    prompt: Option<String>,
    code: Option<String>,
}

impl OAuth2RedirectURI {

    pub fn builder() -> RedirectUriBuilder {
        RedirectUriBuilder::new()
    }
    
    pub fn to_string(&self) -> String {
        let mut uri = format!("{}?client_id={}&redirect_uri={}", self.base_url, self.client_id, self.redirect_uri);

        if let Some(ref scope) = self.scope {
            uri.push_str(&format!("&scope={}", scope));
        }
        if let Some(ref nounce) = self.nounce {
            uri.push_str(&format!("&nounce={}", nounce));
        }
        if let Some(ref state) = self.state {
            uri.push_str(&format!("&state={}", state));
        }
        if let Some(ref client_secret) = self.client_secret {
            uri.push_str(&format!("&client_secret={}", client_secret));
        }
        if let Some(ref prompt) = self.prompt {
            uri.push_str(&format!("&prompt={}", prompt));
        }

        if let Some(ref code) = self.code {
            uri.push_str(&format!("&code={}", code));
        }

        if let Some(ref response_type) = self.response_type {
            uri.push_str(&format!("&response_type={}", response_type));
        }

        uri
    }
}

pub struct RedirectUriBuilder {
    base_url: Option<String>,
    redirect_uri: Option<String>,
    client_id: Option<String>,
    response_type: Option<String>,
    scope: Option<String>,
    nounce: Option<String>,
    state: Option<String>,
    client_secret: Option<String>,
    prompt: Option<String>,
    code: Option<String>,
}

impl RedirectUriBuilder {

    pub fn new() -> Self {
        RedirectUriBuilder { 
            base_url: None,
            redirect_uri: None,
            client_id: None,
            response_type: None,
            scope: None,
            nounce: None,
            state: None,
            client_secret: None,
            prompt: None,
            code: None,
        }
    }

    pub fn client_id(mut self, client_id: &str) -> Self {
        self.client_id = Some(String::from(client_id));
        self
    }

    pub fn redirect_uri(mut self, redirect_uri: &str) -> Self {
        self.redirect_uri = Some(String::from(redirect_uri));
        self
    }

    pub fn base_url(mut self, base_url: &str) -> Self {
        self.base_url = Some(String::from(base_url));
        self
    }

    pub fn response_type(mut self, response_type: &str) -> Self {
        self.response_type = Some(String::from(response_type));
        self
    }

    pub fn scope(mut self, scope: &str) -> Self {
        self.scope = Some(String::from(scope));
        self
    }

    pub fn nounce(mut self, nounce: &str) -> Self {
        self.nounce = Some(String::from(nounce));
        self
    }

    pub fn state(mut self, state: &str) -> Self {
        self.state = Some(String::from(state));
        self
    }

    pub fn client_secret(mut self, client_secret: &str) -> Self {
        self.client_secret = Some(String::from(client_secret));
        self
    }

    pub fn prompt(mut self, prompt: &str) -> Self {
        self.prompt = Some(String::from(prompt));
        self
    }

    pub fn code(mut self, code: &str) -> Self {
        self.code = Some(String::from(code));
        self
    }

    pub fn build(self) -> Result<OAuth2RedirectURI, AppError> {
        if self.base_url.is_none() || 
            self.redirect_uri.is_none() || 
            self.client_id.is_none() {
            return Err(AppError::with_message(ErrorCode::InternalServerError, "base_url, redirect_uri and client_id are required"));
        }

        Ok(OAuth2RedirectURI {
            base_url: self.base_url.expect("base_url is required"),
            redirect_uri: self.redirect_uri.expect("redirect_uri is required"),
            client_id: self.client_id.expect("client_id is required"),
            response_type: self.response_type,
            scope: self.scope,
            nounce: self.nounce,
            state: self.state,
            client_secret: self.client_secret,
            prompt: self.prompt,
            code: self.code,
        })
    }
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn builder_test() {
        let uri = OAuth2RedirectURI::builder()
            .base_url("https://github.com/login")
            .redirect_uri("http://localhost:8080/callback")
            .client_id("client_id")
            .response_type("code")
            .scope("user:email")
            .state("state")
            .build()
            .map_err(|e| {
                panic!("Failed to build OAuth2RedirectURI");
            })
            .unwrap();

        assert_eq!(uri.to_string(), "https://github.com/login?client_id=client_id&redirect_uri=http://localhost:8080/callback&scope=user:email&state=state&response_type=code");
    }
}
