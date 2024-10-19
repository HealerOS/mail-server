use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sea_orm::DatabaseConnection;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn new_server(listener: TcpListener, db: DatabaseConnection) -> Result<Server, std::io::Error> {
    println!("服务启动！！");
    let db_connection = web::Data::new(db);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            .app_data(db_connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
