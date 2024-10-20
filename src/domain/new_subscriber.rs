use crate::domain::subscriber_name::SubscriberUserName;

#[derive(Debug)]
pub struct NewSubscriber {
    pub email: String,
    pub username: SubscriberUserName,
}
