use actix_web::{App, HttpRequest, HttpServer, Responder, get};

#[get("/")]
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
