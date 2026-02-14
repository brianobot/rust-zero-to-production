use sqlx::{Connection, PgConnection, PgPool};
use std::net::TcpListener;

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");

    let port = listener.local_addr().unwrap().port();
    let connection_pool = configure_database().await;
    let server =
        zero_to_production::run(listener, connection_pool).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

async fn configure_database() -> PgPool {
    let mut _connection = PgConnection::connect("postgres://postgres:password@localhost:5432")
        .await
        .expect("Failed to connect to postgres.");

    // sqlx::query(&format!(r#"CREATE DATABASE "{}";"#, "newsletter"))
    //     .execute(&mut connection)
    //     .await
    //     .expect("Failed to create database.");

    let pg_pool = PgPool::connect("postgres://postgres:password@localhost:5432/newsletter")
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&pg_pool)
        .await
        .expect("Failed to migrate the database");

    pg_pool
}

#[tokio::test]
async fn health_check_works() {
    let base_url = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{base_url}/health-check"))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
