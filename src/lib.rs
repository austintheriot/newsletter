use std::net::TcpListener;

use actix_web::{dev::Server, get, post, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

#[get("/health_check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Serialize, Deserialize)]
pub struct SubscribePayload {
    name: String,
    email: String,
}

#[post("/subscribe")]
async fn subscribe(_json: web::Json<SubscribePayload>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check).service(subscribe))
        .listen(listener)?
        .run();

    Ok(server)
}
