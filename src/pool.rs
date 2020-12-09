use crate::{
    config,
    db::DB,
};

type Inner = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

#[derive(Clone)]
pub struct Pool(Inner);

impl Pool {
    pub fn new(db_config: config::DbConfig, pool_config: config::PoolConfig) -> Self {
        use std::time::Duration;

        let url = db_config.get_postgres_url();
        let mut builder = diesel::r2d2::Builder::new();

        if let Some(s) = pool_config.max_size() {
            builder = builder.max_size(s)
        }

        if let Some(s) = pool_config.connection_timeout() {
            builder = builder.connection_timeout(Duration::from_secs(s))
        }

        builder = builder
            .idle_timeout(pool_config.idle_timeout().map(Duration::from_secs))
            .max_lifetime(pool_config.max_lifetime().map(Duration::from_secs))
            .min_idle(pool_config.min_idle());

        Pool(
            builder
                .build(diesel::r2d2::ConnectionManager::new(url))
                .expect("Error creation connection pool"),
        )
    }

    pub fn get_db(&self) -> DB {
        let conn = self.get().unwrap();
        DB::new(conn)
    }
}

impl std::ops::Deref for Pool {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
