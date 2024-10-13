#[derive(serde::Deserialize, Debug)]
pub struct SystemSettings {
    pub db_settings: DBSettings,
    pub application_port: u16,
}
#[derive(serde::Deserialize, Debug)]
pub struct DBSettings {
    pub username: String,
    pub password: String,
    pub database: String,
    pub host: String,
    pub port: u16,
}

pub fn get_config() -> Result<SystemSettings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "./config/configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;
    settings.try_deserialize::<SystemSettings>()
}

impl DBSettings {
    pub fn connection_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database,
        )
    }
    pub fn connection_url_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port,
        )
    }
}
