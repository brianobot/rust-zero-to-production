use actix_web::{App, HttpServer, dev::Server, web};
use std::net::TcpListener;

use crate::routes::health_check::health_check;
use crate::routes::subscriptions::subscriptions;

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health-check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions))
    })
    .listen(listener)?
    .run();
    // .await we do not await the server here

    Ok(server)
}
