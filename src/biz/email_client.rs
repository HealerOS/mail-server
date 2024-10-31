use crate::{domain::subscriber_email::SubscriberEmail, exception::biz_exception::BizResult};
use reqwest::Client;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug)]
pub struct EmailClient {
    pub sender: SubscriberEmail,
    pub http_client: Client,
    pub base_url: String,
    authorization_token: SecretString,
}

#[derive(Serialize, Debug, Deserialize)]
struct SendEmailRequest<'a> {
    from: &'a str,
    to: &'a str,
    subject: &'a str,
    html_body: &'a str,
    text_body: &'a str,
}

impl EmailClient {
    pub async fn send_email(
        &self,
        recipient: &SubscriberEmail,
        subject: String,
        html_content: String,
        text_content: String,
    ) -> BizResult<()> {
        let url = format!("{}/email", self.base_url);

        let request_body = SendEmailRequest {
            from: self.sender.as_ref(),
            to: recipient.as_ref(),
            html_body: &html_content,
            text_body: &text_content,
            subject: &subject,
        };

        self.http_client
            .post(&url)
            .header(
                "X-Postmark-Server-Token",
                self.authorization_token.expose_secret(),
            )
            .json(&request_body)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
    pub fn new(
        base_url: String,
        sender: SubscriberEmail,
        authorization_token: SecretString,
        timeout: Duration,
    ) -> EmailClient {
        EmailClient {
            http_client: Client::builder().timeout(timeout).build().unwrap(),
            base_url,
            sender,
            authorization_token,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::biz::email_client::{EmailClient, SendEmailRequest};
    use crate::domain::subscriber_email::SubscriberEmail;
    use claim::assert_ok;
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::zh_cn::{Paragraph, Sentence};
    use fake::Fake;
    use secrecy::SecretString;
    use std::time::Duration;
    use wiremock::http::Method;
    use wiremock::matchers::{header, header_exists, method, path};
    use wiremock::{Mock, MockServer, Request, ResponseTemplate};

    struct SendEmailBodyMatcher;

    impl wiremock::Match for SendEmailBodyMatcher {
        fn matches(&self, request: &Request) -> bool {
            serde_json::from_slice::<SendEmailRequest>(&request.body)
                .map(|body| {
                    !body.from.is_empty()
                        && !body.to.is_empty()
                        && !body.subject.is_empty()
                        && !body.html_body.is_empty()
                        && !body.text_body.is_empty()
                })
                .unwrap_or(false)
        }
    }

    #[tokio::test]
    async fn send_email_test() {
        // 设置模拟服务器和邮件客户端
        let (mock_server, email_client) = setup_test_server().await;

        // 设置模拟响应
        setup_mock_response(&mock_server).await;

        // 生成测试数据
        let (recipient, subject, content) = generate_test_data();

        // 发送邮件并验证结果
        let response = email_client
            .send_email(recipient, subject, content.clone(), content)
            .await;

        assert_ok!(response);
    }

    async fn setup_test_server() -> (MockServer, EmailClient) {
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let email_client = EmailClient::new(
            mock_server.uri(),
            sender,
            SecretString::new(Box::from("fake token".to_string())),
            Duration::from_millis(200),
        );
        (mock_server, email_client)
    }

    async fn setup_mock_response(mock_server: &MockServer) {
        Mock::given(header_exists("X-Postmark-Server-Token"))
            .and(header("Content-Type", "application/json"))
            .and(path("/email"))
            .and(method(Method::POST))
            .and(SendEmailBodyMatcher)
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(mock_server)
            .await;
    }

    fn generate_test_data() -> (SubscriberEmail, String, String) {
        let recipient = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject = Sentence(1..2).fake();
        let content = Paragraph(1..19).fake();
        (recipient, subject, content)
    }

    #[tokio::test]
    async fn send_email_times_out_if_the_server_takes_too_long() {
        // 设置模拟服务器和邮件客户端
        let (mock_server, email_client) = setup_test_server().await;

        // 设置模拟响应
        Mock::given(header_exists("X-Postmark-Server-Token"))
            .and(header("Content-Type", "application/json"))
            .and(path("/email"))
            .and(method(Method::POST))
            .and(SendEmailBodyMatcher)
            .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(10)))
            .expect(1)
            .mount(&mock_server)
            .await;

        // 生成测试数据
        let (recipient, subject, content) = generate_test_data();

        // 发送邮件并验证结果
        let response = email_client
            .send_email(recipient, subject, content.clone(), content)
            .await;

        assert_ok!(response);
    }
}
