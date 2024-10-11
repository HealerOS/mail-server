pub mod routes;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub fn new_server(listener: TcpListener) -> Result<Server, std::io::Error> {
    println!("服务启动！！");

    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscribe", web::post().to(routes::subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
