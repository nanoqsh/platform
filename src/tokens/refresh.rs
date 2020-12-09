use diesel::{
    backend::Backend,
    deserialize::{self, FromSql},
    serialize::{self, Output, ToSql},
    sql_types::Text,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[sql_type = "Text"]
pub struct RefreshToken(String);

impl RefreshToken {
    pub fn gen() -> Self {
        use rand::{rngs::OsRng, RngCore};

        let mut tok = [0u8; 8];
        OsRng.fill_bytes(&mut tok);

        for b in &mut tok {
            let r = *b % 64;

            *b = match r {
                0..=9 => b'0' + r,
                10..=35 => b'a' + (r - 10),
                36..=61 => b'A' + (r - 36),
                62 => b'-',
                63 => b'_',
                _ => unreachable!(),
            }
        }

        let inner = String::from_utf8(tok.to_vec()).unwrap();
        RefreshToken(inner)
    }

    pub fn ct_eq<R>(&self, rhs: &Self) -> bool {
        use subtle::ConstantTimeEq;

        let left = self.0.as_bytes();
        let right = rhs.0.as_bytes();
        left.ct_eq(right).unwrap_u8() == 1
    }
}

impl<D: Backend> ToSql<Text, D> for RefreshToken
where
    String: ToSql<Text, D>,
{
    fn to_sql<W>(&self, out: &mut Output<W, D>) -> serialize::Result
    where
        W: std::io::Write,
    {
        self.0.to_sql(out)
    }
}

impl<D: Backend> FromSql<Text, D> for RefreshToken
where
    String: FromSql<Text, D>,
{
    fn from_sql(bytes: Option<&D::RawValue>) -> deserialize::Result<RefreshToken> {
        let inner = String::from_sql(bytes)?;
        Ok(RefreshToken(inner))
    }
}
