use std::net::TcpListener;

use newsletter_api::{configuration, startup::run, telemetry};
use secrecy::ExposeSecret;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("newsletter", "info", std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = configuration::get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    let postgres_connection_url = configuration.database.connection_string();
    let db_pool = PgPool::connect(postgres_connection_url.expose_secret())
        .await
        .expect("Failed to connect to Postgres");

    run(listener, db_pool)?.await
}
