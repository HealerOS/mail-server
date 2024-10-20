#[cfg(test)]
mod tests {
    use fake::faker::internet::en::{FreeEmail, Username};
    use fake::Fake;
    use mail_server::configuration::{get_config, DBSettings};
    use mail_server::startup::new_server;
    use mail_server::telemetry::{get_subscriber, init_subscriber};
    use once_cell::sync::Lazy;
    use sea_orm::{Database, DatabaseConnection};
    use secrecy::ExposeSecret;
    use std::net::TcpListener;

    #[tokio::test]
    async fn health_check_succeeds() {
        let url = start_server().await;
        let client = reqwest::Client::new();

        let response = client
            .get(format!("{}/health_check", url))
            .send()
            .await
            .expect("Failed to send request");

        assert!(response.status().is_success());
        assert_eq!(Some(0), response.content_length());
    }

    #[tokio::test]
    async fn subscribe_returns_a_200_for_valid_form_data() {
        let url = start_server().await;

        let client = reqwest::Client::new();
        //生成一个随机的符合规范的用户名和邮箱
        let test_username: String = Username().fake();
        let test_email: String = FreeEmail().fake();

        let test_cases = vec![(
            format!("username={}&email={}", test_username, test_email),
            "应该传入用户名和邮箱",
        )];
        for (invalid_body, _error_message) in test_cases {
            let response = client
                .post(format!("{}/subscribe", url))
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(invalid_body)
                .send()
                .await
                .expect("发送请求失败");

            assert_eq!(response.status().as_u16(), 200,);
        }
    }

    #[tokio::test]
    async fn subscribe_returns_a_400_for_invalid_form_data() {
        let url = start_server().await;
        let client = reqwest::Client::new();

        let test_cases = vec![
            ("username=jason", "没有邮箱！"),
            ("email=gwj@gmail.com", "没有用户名！"),
            ("username=&email=gwj@gmail.com", "用户名不合法"),
            ("username=jason&email=@gmail.com", "邮箱不合法"),
            ("", "啥也没有！"),
        ];
        for (invalid_body, error_message) in test_cases {
            let response = client
                .post(format!("{}/subscribe", url))
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(invalid_body)
                .send()
                .await
                .expect("发送请求失败");

            assert_eq!(
                response.status().as_u16(),
                400,
                "请求错误，与预期不符，错误信息：{}",
                error_message
            );
        }
    }
    pub struct TestApp {
        pub address: String,
    }
    static TRACING: Lazy<()> = Lazy::new(|| {
        let subscriber = get_subscriber("mail-server-test".to_string(), "debug".to_string());
        init_subscriber(subscriber);
    });

    async fn spawn_app() -> TestApp {
        Lazy::force(&TRACING);
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        let configuration = get_config().expect("读取配置失败");
        let db = configure_db(&configuration.db_settings).await;

        let server = new_server(listener, db.clone()).expect("Cannot start server");
        tokio::spawn(server);
        TestApp {
            address: format!("http://127.0.0.1:{}", port),
        }
    }

    async fn start_server() -> String {
        let test_app = spawn_app().await;
        test_app.address
    }

    async fn configure_db(config: &DBSettings) -> DatabaseConnection {
        Database::connect(config.connection_url().expose_secret())
            .await
            .expect("数据库连接失败")
    }
}
