use crate::{tokens::RefreshToken, schema::sessions};
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub refresh_token: RefreshToken,
    pub fingerprint: String,
    pub expires: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "sessions"]
pub struct NewSession<'r, 'f> {
    pub user_id: i32,
    pub refresh_token: &'r RefreshToken,
    pub fingerprint: &'f str,
    pub expires: NaiveDateTime,
}
