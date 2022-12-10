use std::net::TcpListener;

use crate::routes::{health_check, subscribe};
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let socket_addr = listener.local_addr();

    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            // assigns a request_id to each request
            .wrap(TracingLogger::default())
            .service(health_check)
            .service(subscribe)
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    if let Ok(local_addr) = socket_addr {
        println!("Running server at: http://{local_addr}");
    }

    Ok(server)
}
