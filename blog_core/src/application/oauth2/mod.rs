pub mod usecases;
pub mod github;

use rand::distributions::Alphanumeric;
use rand::Rng;
pub use usecases::*;

pub const OAUTH2_AUTHORIZATION_REQUEST_COOKIE_NAME : &str = "oauth2-auth-request";
pub const OAUTH2_REDIRECT_URI_COOKIE_NAME : &str = "redirect-uri";
pub const OAUTH2_MODE_COOKIE_NAME : &str = "mode";
pub const OAUTH2_COOKIE_EXPIRE_SECONDS: u16 = 180;

pub fn generate_rand(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let rand_string: String = (0..length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();
    rand_string
}


