use std::net::TcpListener;

use newsletter_api::run;
use serde_json::json;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{address}/health_check"))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data_json() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let payload = json!({
        "name": "Austin",
        "email": "test@gmail.com",
    });

    let response = client
        .post(format!("{address}/subscribe"))
        .header("Content-Type", "application/json")
        .body(payload.to_string())
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
async fn subscribe_returns_400_for_invalid_form_data_json() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let payloads = [
        json!({
            "cat": "Meow",
            "dog": "woof@gmail.com",
        })
        .to_string(),
        json!({
            "name": "Austin",
        })
        .to_string(),
        json!({
            "email": "test@gmail.com",
        })
        .to_string(),
        String::from(""),
    ];

    for payload in payloads {
        let response = client
            .post(format!("{address}/subscribe"))
            .header("Content-Type", "application/json")
            .body(payload.to_string())
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail with 400 Bad Request when the payload was {payload}"
        );
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");

    let _handle = tokio::spawn(server);
    let address = format!("http://127.0.0.1:{}", port);
    address
}
