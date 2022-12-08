use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SubscribePayload {
    name: String,
    email: String,
}

#[post("/subscribe")]
async fn subscribe(_json: web::Json<SubscribePayload>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
