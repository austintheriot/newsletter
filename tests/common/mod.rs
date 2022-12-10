use std::net::TcpListener;

use newsletter_api::{
    configuration::{get_configuration_with_randomized_database_name, DatabaseSettings},
    startup::run,
    telemetry,
};
use reqwest::Client;
use secrecy::ExposeSecret;
use sqlx::{migrate, Connection, Executor, PgConnection, PgPool};

pub struct TestApp {
    pub address: String,
    pub client: Client,
    pub pool: PgPool,
}

/// Spins up the server with a fresh database to run tests against
pub async fn spawn_app() -> TestApp {
    // only print logs from tests if TEST_LOG flag is set
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = telemetry::get_subscriber("test", "debug", std::io::stdout);
        telemetry::init_subscriber(subscriber);
    } else {
        let subscriber = telemetry::get_subscriber("test", "debug", std::io::sink);
        telemetry::init_subscriber(subscriber);
    }

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let configuration =
        get_configuration_with_randomized_database_name().expect("Failed to read configuration");

    let address = format!("http://127.0.0.1:{}", port);
    let client = reqwest::Client::new();
    let pool = configure_database(&configuration.database).await;
    let server = run(listener, pool.clone()).expect("Failed to bind address");

    // Immediately starts executing. Is dropped when the runtime ends (at the end of tests)
    let _ = tokio::spawn(server);

    TestApp {
        address,
        client,
        pool,
    }
}

/// Spins up a fresh, unique database to run queries against on a per-test basis
pub async fn configure_database(database_settings: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(
        database_settings
            .connection_string_without_db_name()
            .expose_secret(),
    )
    .await
    .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, database_settings.name).as_str())
        .await
        .expect("Failed to create database");

    let db_pool = PgPool::connect(database_settings.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres");

    migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to run database migrations");

    db_pool
}
