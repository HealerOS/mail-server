use crate::domain::new_subscriber::NewSubscriber;

use crate::model::common_response::CommonResponse;
use crate::orm_model::subscriptions;
use actix_web::web::Form;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr};
use serde::Deserialize;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub username: String,
    pub email: String,
}

#[tracing::instrument(name = "正在添加订阅", skip(db))]
pub async fn subscribe(
    user_info: Form<UserInfo>,
    db: web::Data<DatabaseConnection>,
) -> HttpResponse {
    let new_subscriber = match NewSubscriber::try_from(user_info) {
        Ok(subscriber) => subscriber,
        Err(e) => {
            error!("校验信息失败：e{}", e);
            return HttpResponse::BadRequest().json(CommonResponse::error_response(e));
        }
    };

    match insert_subscriber(new_subscriber, db.clone()).await {
        Ok(_) => HttpResponse::Ok().json(CommonResponse::success_response_without_data()),
        Err(e) => {
            error!("error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "正在保存订阅者到DB", skip(db))]
pub async fn insert_subscriber(
    new_subscriber: NewSubscriber,
    db: web::Data<DatabaseConnection>,
) -> Result<(), DbErr> {
    let subscription_user = subscriptions::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        email: ActiveValue::Set(new_subscriber.email.as_ref().to_owned()),
        username: ActiveValue::Set(new_subscriber.username.as_ref().to_string()),
        subscribed_at: ActiveValue::Set(DateTimeWithTimeZone::from(Utc::now())),
    };

    let res = subscription_user.clone().insert(db.get_ref()).await?;

    assert_eq!(subscription_user.id.unwrap(), res.id);
    info!("订阅成功,id:{:?}", res.id);
    Ok(())
}
