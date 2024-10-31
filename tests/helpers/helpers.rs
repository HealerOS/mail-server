use mail_server::boot::server::Application;
use mail_server::config::system_config::get_config;
use mail_server::model::subscriptions::Model;
use mail_server::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;
use sea_orm::{Database, DatabaseConnection};
use secrecy::ExposeSecret;
use wiremock::MockServer;

pub struct TestApp {
    pub address: String,
    pub email_server: MockServer,
    pub db: DatabaseConnection,
}

impl TestApp {
    pub async fn confirm_subscription(&self, p0: &Model) {}
}

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = get_subscriber("mail-server-test".to_string(), "debug".to_string());
    init_subscriber(subscriber);
});

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);
    let email_server = MockServer::start().await;

    //为了保证测试的隔离性，随机化配置
    let system_config = {
        let mut c = get_config().expect("读取配置失败");
        c.application_config.port = 0;
        c.email_config.base_url = email_server.uri();
        c
    };
    let db = Database::connect(system_config.db_settings.connection_url().expose_secret())
        .await
        .unwrap_or_else(|e| {
            panic!(
                "数据库连接失败，error:{},db_url:{}",
                e,
                system_config.db_settings.connection_url().expose_secret()
            )
        });

    let application = Application::build(system_config)
        .await
        .expect("Failed to build application");

    let port = application.port();
    tokio::spawn(application.run_until_stopped());
    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        email_server,
        db,
    }
}

pub async fn start_server() -> TestApp {
    spawn_app().await
}

impl TestApp {
    pub async fn post_subscriptions(&self, body: String) -> reqwest::Response {
        reqwest::Client::new()
            .post(format!("{}/subscribe", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("发送请求失败")
    }
}
