use actix_web::{App, HttpResponse, HttpServer, Responder, dev::Server, web};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run() -> std::io::Result<Server> {
    let server = HttpServer::new(|| App::new().route("/health-check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run();
    // .await we do not await the server here

    Ok(server)
}
