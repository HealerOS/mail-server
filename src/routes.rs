use actix_web::web::Form;
use actix_web::HttpResponse;
use serde::Deserialize;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
#[derive(Debug, Deserialize)]
pub struct UserInfo {
    username: String,
    email: String,
}

pub async fn subscribe(user_info: Form<UserInfo>) -> HttpResponse {
    println!("{:?}", user_info);
    HttpResponse::Ok().finish()
}
