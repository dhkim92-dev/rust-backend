use serde::{Serialize, Deserialize};

use crate ::interfaces::auth::vo::Email;

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String
}

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    pub typ: String,
    pub access_token: String,
    pub refresh_token: String
}
