use std::net::TcpListener;

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero_to_production::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn subscribe_returns_a_200_for_a_valid_form_data() {
    let base_url = spawn_app().await;

    let client = reqwest::Client::new();

    let body = "name=Brian%20Obot&email=brianobot9%40gmail.com";
    let response = client
        .post(&format!("{base_url}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let base_url = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=Brian%20Obot", "missing email"),
        ("email=brianobot9%40gmail.com", "missing name"),
        ("", "missing both name and email"),
    ];

    for (invalid_payload, _error_message) in test_cases {
        let response = client
            .post(format!("{base_url}/subscriptions"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_payload)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            _error_message
        );
    }
}
