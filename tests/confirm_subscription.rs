use crate::helpers::helpers::start_server;
use claim::assert_some;
use fake::faker::internet::en::SafeEmail;
use fake::faker::internet::zh_cn::Username;
use fake::Fake;
use mail_server::model::sea_orm_active_enums::StatusEnum;
use mail_server::model::subscriptions;
use sea_orm::ColumnTrait;
use sea_orm::{EntityTrait, QueryFilter};

mod helpers;

#[tokio::test]
async fn test_confirm_subscription() {
    let app = start_server().await;
    let username: String = Username().fake();
    let email: String = SafeEmail().fake();
    let param = format!("username={}&email={}", username, email);
    app.post_subscriptions(param.to_string()).await;

    let subscriber = subscriptions::Entity::find()
        .filter(subscriptions::Column::Username.eq(username.clone()))
        .one(&app.db)
        .await
        .unwrap();

    let subscriber = assert_some!(subscriber);

    assert_eq!(subscriber.email, email);
    assert_eq!(subscriber.status, StatusEnum::Waiting);

    app.confirm_subscription(&subscriber).await;
}

#[tokio::test]
async fn test_not_confirm_subscription() {}
