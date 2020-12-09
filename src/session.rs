use crate::{
    config::SessionConfig as Config,
    models::{session::NewSession, user::User},
    tokens::*,
};
use chrono::{NaiveDateTime, Utc};
use diesel::PgConnection;
use serde::{Deserialize, Serialize};

pub use secret_key::SecretKey;

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
}

#[derive(Debug)]
pub struct SessionManager {
    access_lifetime: i64,
    offline_lifetime: i64,
    secret_key: SecretKey,
}

impl SessionManager {
    pub fn new(config: Config) -> Self {
        SessionManager {
            access_lifetime: config.access_lifetime().unwrap_or(1800) as i64,
            offline_lifetime: config.offline_lifetime().unwrap_or(604800) as i64,
            secret_key: SecretKey::gen(),
        }
    }

    fn create_refresh_token(&self, user: &User, conn: &PgConnection) -> RefreshToken {
        use super::schema::sessions;
        use diesel::prelude::*;

        let refresh_token = RefreshToken::gen();
        let expires_secs = Utc::now().timestamp() + self.offline_lifetime;
        let expires = NaiveDateTime::from_timestamp(expires_secs, 0);

        let new = NewSession {
            user_id: user.id,
            refresh_token: &refresh_token,
            fingerprint: "fingerprint",
            expires,
        };

        let inserted_rows = diesel::insert_into(sessions::table)
            .values(new)
            .execute(conn)
            .expect("Failed");

        assert_eq!(inserted_rows, 1);

        refresh_token
    }

    fn create_access_token(&self, user: &User) -> AccessToken {
        let expires = Utc::now().timestamp() + self.access_lifetime;
        AccessToken::new(user, expires, &self.secret_key)
    }

    pub fn create_session(&self, user: &User, conn: &PgConnection) -> Session {
        let access_token = self.create_access_token(user);
        let refresh_token = self.create_refresh_token(user, conn);

        Session {
            access_token,
            refresh_token,
        }
    }
}

mod secret_key {
    #[derive(Debug)]
    pub struct SecretKey([u8; 32]);

    impl SecretKey {
        pub fn gen() -> Self {
            use rand::RngCore;

            let mut key = [0u8; 32];
            rand::rngs::OsRng.fill_bytes(&mut key);

            SecretKey(key)
        }
    }

    impl std::ops::Deref for SecretKey {
        type Target = [u8];

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}
