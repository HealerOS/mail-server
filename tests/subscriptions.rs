use crate::helpers::helpers::start_server;
use fake::faker::internet::en::{FreeEmail, Username};
use fake::Fake;

mod helpers;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = start_server().await;
    reqwest::Client::new();
    //生成一个随机的符合规范的用户名和邮箱
    let test_username: String = Username().fake();
    let test_email: String = FreeEmail().fake();

    let test_cases = vec![(
        format!("username={}&email={}", test_username, test_email),
        "应该传入用户名和邮箱",
    )];
    for (invalid_body, _error_message) in test_cases {
        let response = app.post_subscriptions(invalid_body).await;

        assert_eq!(response.status().as_u16(), 200,);
    }
}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_form_data() {
    let app = start_server().await;

    let test_cases = vec![
        ("username=jason", "没有邮箱！"),
        ("email=gwj@gmail.com", "没有用户名！"),
        ("username=&email=gwj@gmail.com", "用户名不合法"),
        ("username=jason&email=@gmail.com", "邮箱不合法"),
        ("", "啥也没有！"),
    ];
    for (invalid_body, error_message) in test_cases {
        let response = app.post_subscriptions(invalid_body.to_string()).await;

        assert_eq!(
            response.status().as_u16(),
            400,
            "请求错误，与预期不符，错误信息：{}",
            error_message
        );
    }
}
