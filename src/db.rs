use crate::{
    models::user::{ConstraintError, NewUser, User},
    schema::users,
};
use diesel::{prelude::*, r2d2, result::Error};

type Pooled = r2d2::PooledConnection<r2d2::ConnectionManager<diesel::PgConnection>>;

pub struct DB {
    conn: Pooled,
}

impl DB {
    pub fn new(conn: Pooled) -> Self {
        DB { conn }
    }

    pub fn make_user(&self, new: &NewUser) -> Result<User, ConstraintError> {
        new.check_constraints()?;

        let result = diesel::insert_into(users::table)
            .values(new)
            .get_result(&self.conn);

        Ok(match result {
            Ok(user) => user,
            Err(e) => match e {
                Error::DatabaseError(_, info) => panic!("Database error: {:?}", info),
                e => panic!(e),
            },
        })
    }

    pub fn get_user(&self, login: &str) -> Option<User> {
        let result = users::table
            .filter(users::login.eq(login))
            .get_result(&self.conn)
            .optional();

        match result {
            Ok(user) => user,
            Err(e) => match e {
                Error::DatabaseError(_, info) => panic!("Database error: {:?}", info),
                e => panic!(e),
            },
        }
    }
}
