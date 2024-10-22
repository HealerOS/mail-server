use mail_server::biz::email_client::EmailClint;
use mail_server::boot::server::new_server;
use mail_server::config::system_config::get_config;
use mail_server::telemetry::{get_subscriber, init_subscriber};
use sea_orm::Database;
use secrecy::ExposeSecret;
use std::net::TcpListener;
use tracing::info;

#[warn(clippy::all, clippy::pedantic)]
#[tokio::main]
//todo 后续重构优化这里
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("mail-server".to_string(), "info".to_string());
    init_subscriber(subscriber);

    let system_config = get_config().expect("读取系统配置失败");

    info!("系统配置是:{:?}", system_config);
    let db = Database::connect(system_config.db_settings.connection_url().expose_secret())
        .await
        .unwrap_or_else(|e| {
            panic!(
                "数据库连接失败，error:{},db_url:{}",
                e,
                system_config.db_settings.connection_url().expose_secret()
            )
        });

    let listener = TcpListener::bind(format!(
        "{}:{}",
        system_config.application_config.host, system_config.application_config.port
    ))?;
    let sender_email = system_config
        .email_config
        .sender_email()
        .expect("获取发送者email失败");

    let email_client = EmailClint::new(system_config.email_config.base_url, sender_email);

    new_server(listener, db, email_client)?.await
}
