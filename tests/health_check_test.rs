#[cfg(test)]
mod tests {
    use mail_server::configuration::{get_config, DBSettings};
    use mail_server::startup::new_server;
    use sqlx::{Connection, Executor, PgConnection, PgPool};
    use std::net::TcpListener;
    use uuid::Uuid;

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

        let configuration = get_config().expect("读取配置失败");

        let _db_connection = PgConnection::connect(&configuration.db_settings.connection_url())
            .await
            .expect("连接DB失败");
        let client = reqwest::Client::new();

        let test_cases = vec![("username=jason&email=gwj@gmail.com", "应该传入用户名和邮箱")];
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
        pub db_pool: PgPool,
    }

    async fn spawn_app() -> TestApp {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        let mut configuration = get_config().expect("读取配置失败");
        configuration.db_settings.database = Uuid::new_v4().to_string();
        let db_connection_pool = configure_db(&configuration.db_settings).await;

        let server = new_server(listener, db_connection_pool.clone()).expect("Cannot start server");
        tokio::spawn(server);
        TestApp {
            address: format!("http://127.0.0.1:{}", port),
            db_pool: db_connection_pool,
        }
    }

    async fn start_server() -> String {
        let test_app = spawn_app().await;
        test_app.address
    }

    async fn configure_db(config: &DBSettings) -> PgPool {
        let mut connection = PgConnection::connect(&config.connection_url_without_db())
            .await
            .expect("获取数据库服务器连接失败");
        //创建集成测试DB
        connection
            .execute(format!(r#"create database "{}";"#, config.database).as_str())
            .await
            .expect("创建db失败");
        //迁移db
        let connection_pool = PgPool::connect(&config.connection_url())
            .await
            .expect("连接到数据库失败");

        sqlx::migrate!("./migrations")
            .run(&connection_pool)
            .await
            .expect("迁移数据库失败");

        connection_pool
    }
}
