use actix_web::{get, HttpResponse};

#[get("/health_check")]
async fn health_check() -> HttpResponse {
    log::info!("Health check endpoint hit");
    HttpResponse::Ok().finish()
}
