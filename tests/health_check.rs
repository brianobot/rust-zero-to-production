use std::net::TcpListener;

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

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero_to_production::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
