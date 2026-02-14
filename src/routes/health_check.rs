use actix_web::{HttpResponse, Responder};
use uuid::Uuid;

pub async fn health_check() -> impl Responder {
    let request_id = Uuid::new_v4();

    let request_span = tracing::info_span!(
        "Running the Health Check Route Function.",
        %request_id,
    );
    let _request_span_guard = request_span.enter();

    HttpResponse::Ok()
}
