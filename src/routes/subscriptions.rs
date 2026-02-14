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
        Ok(_) => HttpResponse::Ok(),
        Err(e) => {
            println!("Failed to execute query: {e}");
            HttpResponse::InternalServerError()
        }
    }
}
