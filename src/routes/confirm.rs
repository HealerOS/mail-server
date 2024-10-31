use actix_web::HttpResponse;

#[tracing::instrument(name = "确认订阅")]
pub async fn confirm(_subscription_token: String) -> HttpResponse {
    HttpResponse::Ok().finish()
}
