use crate::schema::users;
pub use password::Password;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub email: String,
    pub password: String,
}

pub enum ConstraintError {
    LoginLen(usize),
    EmailLen(usize),
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub login: String,
    pub email: String,
    pub password: Password,
}

impl NewUser {
    pub fn check_constraints(&self) -> Result<(), ConstraintError> {
        if let 1..=250 = self.login.len() {
            return Err(ConstraintError::LoginLen(self.login.len()));
        }

        if let 1..=250 = self.email.len() {
            return Err(ConstraintError::EmailLen(self.email.len()));
        }

        Ok(())
    }
}

mod password {
    use diesel::{
        backend::Backend,
        deserialize::{self, FromSql},
        serialize::{self, Output, ToSql},
        sql_types::Text,
    };
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::{error::Error, io};

    #[derive(Clone, AsExpression, FromSqlRow)]
    #[sql_type = "Text"]
    pub enum Password {
        Raw(String),
        Hashed(String),
    }

    impl std::fmt::Debug for Password {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let pass = match self {
                Password::Raw(_) => "..",
                Password::Hashed(hashed) => hashed.as_str(),
            };

            write!(f, "\"{}\"", pass)
        }
    }

    const ERROR_MESSAGE: &str = "Unable to insert unhashed password";

    #[derive(Debug)]
    struct UnhashedError;

    impl std::fmt::Display for UnhashedError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", ERROR_MESSAGE)
        }
    }

    impl Error for UnhashedError {}

    impl<D: Backend> ToSql<Text, D> for Password
    where
        String: ToSql<Text, D>,
    {
        fn to_sql<W>(&self, out: &mut Output<W, D>) -> serialize::Result
        where
            W: io::Write,
        {
            let hashed = match self {
                Password::Raw(_) => return Err(Box::new(UnhashedError)),
                Password::Hashed(h) => h,
            };

            hashed.to_sql(out)
        }
    }

    impl<D: Backend> FromSql<Text, D> for Password
    where
        String: FromSql<Text, D>,
    {
        fn from_sql(bytes: Option<&D::RawValue>) -> deserialize::Result<Password> {
            let hashed = String::from_sql(bytes)?;
            Ok(Password::Hashed(hashed))
        }
    }

    impl Serialize for Password {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            use serde::ser::Error as Se;

            let hashed = match self {
                Password::Raw(_) => return Err(Se::custom(ERROR_MESSAGE)),
                Password::Hashed(h) => h.as_str(),
            };

            serializer.serialize_str(hashed)
        }
    }

    impl<'d> Deserialize<'d> for Password {
        fn deserialize<D>(deserializer: D) -> Result<Password, D::Error>
        where
            D: Deserializer<'d>,
        {
            let hashed = Deserialize::deserialize(deserializer)?;
            Ok(Password::Raw(hashed))
        }
    }
}
