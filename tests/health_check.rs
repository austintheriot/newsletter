use newsletter_api::run;

#[tokio::test]
async fn health_check_works() {
    spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

fn spawn_app() {
    let server = run("127.0.0.1:8000").expect("Failed to bind address");

    let _handle = tokio::spawn(server);
}
