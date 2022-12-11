use std::{net::TcpListener, time::Duration};

use newsletter::{configuration, startup::run, telemetry};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("newsletter", "info", std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = configuration::get_configuration().expect("Failed to read configuration");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    let postgres_connection_url = configuration.database.connection_string();
    let db_pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(2))
        .connect_lazy(postgres_connection_url.expose_secret())
        .expect("Failed to connect to Postgres");

    run(listener, db_pool)?.await
}
