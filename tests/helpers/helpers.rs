use mail_server::boot::server::Application;
use mail_server::config::system_config::get_config;
use mail_server::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;

pub struct TestApp {
    pub address: String,
}
static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = get_subscriber("mail-server-test".to_string(), "debug".to_string());
    init_subscriber(subscriber);
});

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    //为了保证测试的隔离性，随机化配置
    let system_config = {
        let mut c = get_config().expect("读取配置失败");
        c.application_config.port = 0;
        c
    };

    let application = Application::build(system_config)
        .await
        .expect("Failed to build application");

    let port = application.port();
    tokio::spawn(application.run_until_stopped());
    TestApp {
        address: format!("http://127.0.0.1:{}", port),
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
