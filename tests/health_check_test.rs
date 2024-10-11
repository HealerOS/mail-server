#[cfg(test)]
mod tests {
    use std::net::TcpListener;

    #[tokio::test]
    async fn health_check_succeeds() {
        let url = start_server();
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
        let url = start_server();
        let client = reqwest::Client::new();

        let test_cases = vec![("username=jason&email=gwj@gmail.com", "应该传入用户名和邮箱")];
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
                200,
                "请求错误，与预期不符，错误信息：{}",
                error_message
            );
        }
    }

    #[tokio::test]
    async fn subscribe_returns_a_400_for_invalid_form_data() {
        let url = start_server();
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

    fn spawn_app() -> u16 {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        let server = mail_server::new_server(listener).expect("Cannot start server");
        let _ = tokio::spawn(server);
        port
    }

    fn start_server() -> String {
        let port = spawn_app();
        format!("http://localhost:{}", port)
    }
}
