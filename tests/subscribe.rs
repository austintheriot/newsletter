mod common;

use common::spawn_app;

use newsletter_api::configuration::get_configuration;
use serde_json::json;
use sqlx::{Connection, PgConnection};

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data_json() {
    let address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");
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

    assert_eq!(response.status().as_u16(), 200);

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "test@gmail.com");
    assert_eq!(saved.name, "Austin");
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
