use crate::{domain::subscriber_email::SubscriberEmail, exception::biz_exception::BizResult};
use reqwest::Client;

pub struct EmailClint {
    pub sender: SubscriberEmail,
    pub http_client: Client,
    pub base_url: String,
}

impl EmailClint {
    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: String,
        html_content: String,
        text_content: String,
    ) -> BizResult<()> {
        todo!()
    }
    pub fn new(base_url: String, sender: SubscriberEmail) -> EmailClint {
        EmailClint {
            base_url,
            sender,
            http_client: Client::new(),
        }
    }
}
