use crate::interfaces::auth::dto::LoginResponse;

pub struct LoginCommand  {
    pub principal: String,
    pub credential: String
}

pub  struct  LoginCommandResponse  {
    pub typ:String,
    pub access_token: String,
    pub refresh_token: String
}

impl LoginCommandResponse {
    fn to_response(&self) -> LoginResponse {
        LoginResponse {
            typ: self.typ.clone(),
            access_token: self.access_token.clone(),
            refresh_token: self.refresh_token.clone()
        }
    }
}
