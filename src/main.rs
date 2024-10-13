use mail_server::configuration::get_config;
use mail_server::startup::new_server;
use sqlx::{Connection, PgPool};
use std::net::TcpListener;

#[warn(clippy::all, clippy::pedantic)]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let system_config = get_config().expect("读取系统配置失败");

    let configuration = get_config().expect("读取配置失败");

    let db_connection_pool = PgPool::connect(&configuration.db_settings.connection_url())
        .await
        .expect("连接DB失败");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", system_config.application_port))?;
    new_server(listener, db_connection_pool)?.await
}
