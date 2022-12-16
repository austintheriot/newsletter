mod common;

use common::spawn_app;

use serde_json::json;

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data_json() {
    let app = spawn_app().await;
    let payload = json!({
        "name": "Austin",
        "email": "test@gmail.com",
    });

    let response = app
        .client
        .post(format!("{}/subscribe", app.address))
        .header("Content-Type", "application/json")
        .body(payload.to_string())
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status().as_u16(), 200);

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "test@gmail.com");
    assert_eq!(saved.name, "Austin");
}

#[tokio::test]
async fn subscribe_returns_400_for_invalid_form_data_json() {
    let app = spawn_app().await;

    let payloads = [
        (
            json!({
                "cat": "Meow",
                "dog": "woof@gmail.com",
            })
            .to_string(),
            "wrong properties",
        ),
        (
            json!({
                "name": "Austin",
            })
            .to_string(),
            "missing email",
        ),
        (
            json!({
                "email": "test@gmail.com",
            })
            .to_string(),
            "missing name",
        ),
        (
            json!({
                "name": "",
                "email": "test@gmail.com",
            })
            .to_string(),
            "name empty",
        ),
        (
            json!({
                "name": "Test",
                "email": "",
            })
            .to_string(),
            "email empty",
        ),
        (
            json!({
                "name": "\\",
                "email": "test@gmail.com",
            })
            .to_string(),
            "invalid name",
        ),
        (String::from(""), "completely empty body"),
    ];

    for (payload, reason_for_rejection) in payloads {
        let response = app
            .client
            .post(format!("{}/subscribe", app.address))
            .header("Content-Type", "application/json")
            .body(payload.to_string())
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail with 400 Bad Request when the payload was {payload}. It should have rejected the payload due to {reason_for_rejection}"
        );
    }
}
