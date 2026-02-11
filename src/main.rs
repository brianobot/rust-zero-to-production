use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, web};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/")]
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("Hello {}!", &name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
            .route("/health-check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
