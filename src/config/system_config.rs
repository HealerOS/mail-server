use crate::domain::subscriber_email::SubscriberEmail;
use crate::exception::biz_exception::BizResult;
use secrecy::{ExposeSecret, SecretString};
use std::env;
use std::time::Duration;

#[derive(serde::Deserialize, Debug)]
pub struct SystemConfig {
    pub db_settings: DBSettings,
    pub application_config: ApplicationConfig,
    pub email_config: EmailConfig,
}
#[derive(serde::Deserialize, Debug)]
pub struct EmailConfig {
    pub base_url: String,
    pub sender_email: String,
    pub authorization_token: SecretString,
    pub timeout_milliseconds: u64,
}

#[derive(serde::Deserialize, Debug)]
pub struct ApplicationConfig {
    pub host: String,
    pub port: u16,
}

#[derive(serde::Deserialize, Debug)]
pub struct DBSettings {
    pub username: String,
    pub password: SecretString,
    pub database: String,
    pub host: String,
    pub port: u16,
}

pub fn get_config() -> Result<SystemConfig, config::ConfigError> {
    const BASE_DIR_FILE_NAME: &str = "base.yaml";
    let cur_dir = env::current_dir().expect("Failed to determine current directory");
    let config_dir = cur_dir.join("config");
    let environment = env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "local".into());
    let environment_config_file_name = format!("{}.yaml", environment);

    let settings = config::Config::builder()
        .add_source(config::File::from(config_dir.join(BASE_DIR_FILE_NAME)))
        .add_source(config::File::from(
            config_dir.join(environment_config_file_name),
        ))
        .build()?;
    settings.try_deserialize::<SystemConfig>()
}

impl DBSettings {
    pub fn connection_url(&self) -> SecretString {
        let mut db_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database,
        );
        if env::var("DATABASE_URL").is_ok() {
            db_url = env::var("DATABASE_URL").unwrap();
        }
        SecretString::from(db_url)
    }
}

impl EmailConfig {
    pub fn sender_email(&self) -> BizResult<SubscriberEmail> {
        SubscriberEmail::parse(self.sender_email.clone())
    }
    pub fn timeout(&self) -> Duration {
        Duration::from_millis(self.timeout_milliseconds)
    }
}
