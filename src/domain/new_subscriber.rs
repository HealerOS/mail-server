use crate::domain::subscriber_email::SubscriberEmail;
use crate::domain::subscriber_name::SubscriberUserName;
use crate::exception::biz_exception::BizError;
use crate::routes::UserInfo;
use actix_web::web::Form;

#[derive(Debug)]
pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub username: SubscriberUserName,
}

impl TryFrom<Form<UserInfo>> for NewSubscriber {
    type Error = BizError;

    fn try_from(user_info: Form<UserInfo>) -> Result<Self, BizError> {
        let subscriber_username = SubscriberUserName::parse(user_info.0.username)?;
        let subscriber_email = SubscriberEmail::parse(user_info.0.email)?;

        let new_subscriber = NewSubscriber {
            email: subscriber_email,
            username: subscriber_username,
        };
        Ok(new_subscriber)
    }
}
