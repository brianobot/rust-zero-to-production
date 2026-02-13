use actix_web::{HttpResponse, Responder, web::Form};

#[derive(serde::Deserialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
}

pub async fn subscriptions(_form: Form<FormData>) -> impl Responder {
    HttpResponse::Ok()
}
