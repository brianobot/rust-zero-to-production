use actix_web::{
    HttpResponse, Responder,
    web::{self, Form},
};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
}

pub async fn subscriptions(
    _form: Form<FormData>,
    _connection: web::Data<PgPool>,
) -> impl Responder {
    let request_id = Uuid::new_v4();

    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %_form.email,
        subscriber_name= %_form.name
    );
    let _request_span_guard = request_span.enter();

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        _form.email,
        _form.name,
        Utc::now(),
    )
    .execute(_connection.get_ref())
    .await
    {
        Ok(_) => {
            tracing::info!("Request id {request_id} New Subscriber added in the database");
            HttpResponse::Ok()
        }
        Err(e) => {
            tracing::error!("Request id {request_id} Failed to execute query: {e:?}");
            HttpResponse::InternalServerError()
        }
    }
}
