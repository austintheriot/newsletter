use std::net::TcpListener;

use newsletter_api::{configuration::get_configuration, startup::run};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    let postgres_connection_url = configuration.database.connection_string();
    let db_pool = PgPool::connect(&postgres_connection_url)
        .await
        .expect("Failed to connect to Postgres");

    run(listener, db_pool)?.await
}
