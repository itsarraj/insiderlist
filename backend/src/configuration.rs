#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub email: EmailSettings,
    pub application_host: String,
    pub application_port: u16,
    #[serde(default)]
    pub rate_limit: RateLimitSettings,
}

#[derive(serde::Deserialize, Clone)]
#[serde(default)]
pub struct RateLimitSettings {
    /// Refill budget for `POST /subscribe` per client IP per minute.
    pub subscribe_requests_per_minute: u64,
    /// Maximum burst of subscribe requests before the per-minute refill applies.
    pub subscribe_burst: u32,
}

impl Default for RateLimitSettings {
    fn default() -> Self {
        Self {
            subscribe_requests_per_minute: 8,
            subscribe_burst: 5,
        }
    }
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct EmailSettings {
    pub api_key: String,
    pub from_email: String,
    pub from_name: String,
    /// Display name for the product (e.g. InsiderList) in transactional emails.
    pub product_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new("configuration.yaml", config::FileFormat::Yaml).required(false))
        .add_source(config::Environment::default().prefix("APP").separator("__"))
        .build()?;
    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}
