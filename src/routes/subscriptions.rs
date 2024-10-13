use actix_web::web::Form;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::types::chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    username: String,
    email: String,
}

pub async fn subscribe(
    user_info: Form<UserInfo>,
    connection_pool: web::Data<PgPool>,
) -> HttpResponse {
    println!("{:?}", user_info);

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id,username, email,subscribed_at)
        VALUES ($1,$2,$3,$4)
        "#,
        Uuid::new_v4(),
        user_info.username,
        user_info.email,
        //todo 这里时间好像没有带时区就插入了
        Utc::now()
    )
    .execute(connection_pool.get_ref())
    .await
    {
        Ok(_) => {
            println!("插入成功：{:?}！", user_info);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            println!("插入失败，error:{}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
