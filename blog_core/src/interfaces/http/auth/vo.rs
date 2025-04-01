use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Email {
    pub address: String
}

impl Email {
    pub fn new(address: String) -> Self {
        Email { address }
    }
}

impl TryFrom<String> for Email {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err("Email cannot be empty".to_string())
        } else {
            Ok(Email::new(value))
        }
    }
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.address
    }
}
