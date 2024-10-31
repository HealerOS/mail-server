use crate::helpers::helpers::start_server;
use claim::assert_some;
use fake::faker::internet::en::SafeEmail;
use fake::faker::internet::zh_cn::Username;
use fake::Fake;
use mail_server::model::sea_orm_active_enums::StatusEnum;
use mail_server::model::subscriptions;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde_json::Value;
use wiremock::http::Method;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

mod helpers;

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

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_for_valid_data() {
    let app = start_server().await;
    let username: String = Username().fake();
    let email: String = SafeEmail().fake();
    let test_cases = vec![format!("username={}&email={}", username, email)];

    Mock::given(path("/email"))
        .and(method(Method::POST))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    for request_body in test_cases {
        let response = app.post_subscriptions(request_body.to_string()).await;
        assert_eq!(response.status().as_u16(), 200,);
    }

    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let request_body: Value = serde_json::from_slice(&email_request.body).unwrap();

    let get_link = |s: &str| {
        let links = linkify::LinkFinder::new()
            .links(s)
            .filter(|l| *l.kind() == linkify::LinkKind::Url)
            .collect::<Vec<_>>();
        links[0].as_str().to_owned()
    };

    let html_link = get_link(request_body["html_body"].as_str().unwrap());
    let text_link = get_link(request_body["text_body"].as_str().unwrap());
    assert_eq!(html_link, text_link);
    let saved_subscriber = subscriptions::Entity::find()
        .filter(subscriptions::Column::Email.eq(&email))
        .one(&app.db)
        .await
        .unwrap();
    let saved_subscriber = assert_some!(saved_subscriber);
    assert_eq!(saved_subscriber.email, email);
    assert_eq!(saved_subscriber.username, username);
    assert_eq!(saved_subscriber.status, StatusEnum::Waiting);
}
