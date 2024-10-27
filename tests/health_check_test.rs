use crate::helpers::helpers::start_server;

mod helpers;

#[tokio::test]
async fn health_check_succeeds() {
    let app = start_server().await;
    let url = app.address;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", url))
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
