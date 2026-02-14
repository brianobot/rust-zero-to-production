use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, dev::Server, web};
use sqlx::PgPool;
use std::net::TcpListener;

use crate::routes::health_check::health_check;
use crate::routes::subscriptions::subscriptions;

pub fn run(listener: TcpListener, connection_pool: PgPool) -> std::io::Result<Server> {
    let connection_pool = web::Data::new(connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health-check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions))
            .app_data(connection_pool.clone())
    })
    .listen(listener)?
    .run();
    // .await we do not await the server here

    Ok(server)
}
