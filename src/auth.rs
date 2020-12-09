use crate::{config::ScryptConfig as Config, db::DB, models::user::*};

#[derive(serde::Deserialize)]
pub struct SignIn {
    login: String,
    password: String,
}

pub enum Failed {
    UnknownLogin,
    IncorrectPassword,
}

pub struct Auth {
    params: scrypt::ScryptParams,
}

impl Auth {
    pub fn new(config: Option<Config>) -> Self {
        Auth {
            params: config
                .map(|c| scrypt::ScryptParams::new(c.log_n(), c.r(), c.p()))
                .unwrap_or_else(|| Ok(scrypt::ScryptParams::recommended()))
                .expect("Invalid scrypt parameters!"),
        }
    }

    fn hash_password(&self, password: Password) -> Password {
        Password::Hashed(match password {
            Password::Raw(raw) => {
                scrypt::scrypt_simple(&raw, &self.params).expect("Failed password hashing")
            }
            Password::Hashed(_) => panic!("Password already hashed!"),
        })
    }

    fn check_password(&self, password: &str, hashed: &str) -> bool {
        use scrypt::errors::CheckError;

        match scrypt::scrypt_check(password, hashed) {
            Ok(_) => true,
            Err(CheckError::HashMismatch) => false,
            Err(CheckError::InvalidFormat) => panic!("Invalid format of the hash string"),
        }
    }

    pub fn sign_up(&self, user: NewUser, db: &DB) -> Result<User, ConstraintError> {
        let user = NewUser {
            password: self.hash_password(user.password),
            ..user
        };

        db.make_user(&user)
    }

    pub fn sign_in(&self, form: &SignIn, db: &DB) -> Result<User, Failed> {
        if let Some(user) = db.get_user(form.login.as_str()) {
            if self.check_password(form.password.as_str(), user.password.as_str()) {
                Ok(user)
            } else {
                Err(Failed::IncorrectPassword)
            }
        } else {
            Err(Failed::UnknownLogin)
        }
    }
}
