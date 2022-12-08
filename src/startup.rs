use std::net::TcpListener;

use crate::routes::{health_check, subscribe};
use actix_web::{dev::Server, App, HttpServer};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let socket_addr = listener.local_addr();

    let server = HttpServer::new(|| App::new().service(health_check).service(subscribe))
        .listen(listener)?
        .run();

    if let Ok(local_addr) = socket_addr {
        println!("Running server at: http://{local_addr}");
    }

    Ok(server)
}
