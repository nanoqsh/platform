use crate::{auth::Auth, config::Config, pool::Pool, session::SessionManager};

pub struct Server {
    pool: Pool,
    auth: Auth,
    session: SessionManager,
}

impl Server {
    pub fn new(config: Config) -> Self {
        let Config {
            server,
            db,
            pool,
            scrypt,
            session,
        } = config;

        let _ = server;

        Server {
            pool: Pool::new(db, pool),
            auth: Auth::new(scrypt),
            session: SessionManager::new(session),
        }
    }

    pub fn pool(&self) -> &Pool {
        &self.pool
    }

    pub fn auth(&self) -> &Auth {
        &self.auth
    }

    pub fn session(&self) -> &SessionManager {
        &self.session
    }
}
