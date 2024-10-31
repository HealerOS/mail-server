use crate::biz::email_client::EmailClient;
use crate::config::system_config::SystemConfig;
use crate::routes::{confirm, health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sea_orm::Database;
use secrecy::ExposeSecret;
use std::net::TcpListener;
use tracing::info;
use tracing_actix_web::TracingLogger;

pub struct Application {
    port: u16,
    server: Server,
}
impl Application {
    pub async fn build(system_config: SystemConfig) -> Result<Self, std::io::Error> {
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
        let port = listener.local_addr()?.port();
        let sender_email = system_config
            .email_config
            .sender_email()
            .expect("获取发送者email失败");

        let timeout = system_config.email_config.timeout();
        let email_client = EmailClient::new(
            system_config.email_config.base_url,
            sender_email,
            system_config.email_config.authorization_token,
            timeout,
        );

        let db_connection = web::Data::new(db);
        let email_clint = web::Data::new(email_client);
        let server = HttpServer::new(move || {
            App::new()
                .wrap(TracingLogger::default())
                .route("/health_check", web::get().to(health_check))
                .route("/subscribe", web::post().to(subscribe))
                .route("/confirm", web::get().to(confirm))
                .app_data(db_connection.clone())
                .app_data(email_clint.clone())
        })
        .listen(listener)?
        .run();
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}
