use std::net;

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run<A: net::ToSocketAddrs>(addr: A) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind(addr)?
        .run();

    Ok(server)
}
