#[derive(Debug, Default, serde::Deserialize)]
pub struct HttpServerConfig {
    pub port: Option<u16>,
    pub host: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct DbConfig {
    host: String,
    database: String,
    user: String,
    password: String,
}

impl DbConfig {
    pub fn get_postgres_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}/{}",
            self.user, self.password, self.host, self.database
        )
    }
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct PoolConfig {
    max_size: Option<u32>,
    connection_timeout: Option<u64>,
    idle_timeout: Option<u64>,
    max_lifetime: Option<u64>,
    min_idle: Option<u32>,
}

impl PoolConfig {
    pub fn max_size(&self) -> Option<u32> {
        self.max_size
    }

    pub fn connection_timeout(&self) -> Option<u64> {
        self.connection_timeout
    }

    pub fn idle_timeout(&self) -> Option<u64> {
        self.idle_timeout
    }

    pub fn max_lifetime(&self) -> Option<u64> {
        self.max_lifetime
    }

    pub fn min_idle(&self) -> Option<u32> {
        self.min_idle
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct ScryptConfig {
    log_n: u8,
    r: u32,
    p: u32,
}

impl ScryptConfig {
    pub fn log_n(&self) -> u8 {
        self.log_n
    }
    pub fn r(&self) -> u32 {
        self.r
    }
    pub fn p(&self) -> u32 {
        self.p
    }
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct SessionConfig {
    access_lifetime: Option<u32>,
    offline_lifetime: Option<u32>,
}

impl SessionConfig {
    pub fn access_lifetime(&self) -> Option<u32> {
        self.access_lifetime
    }

    pub fn offline_lifetime(&self) -> Option<u32> {
        self.offline_lifetime
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    #[serde(default)]
    pub server: HttpServerConfig,
    pub db: DbConfig,

    #[serde(default)]
    pub pool: PoolConfig,
    pub scrypt: Option<ScryptConfig>,

    #[serde(default)]
    pub session: SessionConfig,
}

impl Config {
    pub fn load() -> Self {
        const PATH: &str = "./config.toml";

        let content = match std::fs::read_to_string(PATH) {
            Ok(content) => content,
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    panic!("Config file {} not found!", PATH)
                } else {
                    panic!("Failed to read config file ({}) {}", PATH, e)
                }
            }
        };

        match toml::from_str(content.as_str()) {
            Ok(config) => config,
            Err(e) => panic!("Failed to read {}: {}", PATH, e),
        }
    }
}
