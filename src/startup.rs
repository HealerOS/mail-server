use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::{Pool, Postgres};
use std::net::TcpListener;

pub fn new_server(
    listener: TcpListener,
    db_connection: Pool<Postgres>,
) -> Result<Server, std::io::Error> {
    println!("服务启动！！");
    let db_connection = web::Data::new(db_connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            .app_data(db_connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
