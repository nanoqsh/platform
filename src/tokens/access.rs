use crate::{models::user::User, session::SecretKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims<'l> {
    user_id: i32,
    login: &'l str,
    exp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessToken(String);

impl AccessToken {
    pub fn new(user: &User, expires: i64, secret_key: &SecretKey) -> Self {
        let claims = Claims {
            user_id: user.id,
            login: user.login.as_str(),
            exp: expires,
        };

        let token = jwt::encode(
            &jwt::Header::default(),
            &claims,
            &jwt::EncodingKey::from_secret(secret_key),
        )
        .expect("Failed creating access token");

        AccessToken(token)
    }
}

impl std::ops::Deref for AccessToken {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}
