use sqlx::PgPool;
use std::net::TcpListener;
use zero_to_production::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to Bind to port");
    let connection_pool = PgPool::connect("postgres://postgres:password@localhost:5432/newsletter")
        .await
        .expect("Failed to connect to postgres");

    run(listener, connection_pool)?.await
}
