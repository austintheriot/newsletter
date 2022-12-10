use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct SubscribePayload {
    name: String,
    email: String,
}

#[post("/subscribe")]
async fn subscribe(json: web::Json<SubscribePayload>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    log::info!(
        "request_id {request_id} Adding {} {} as a new subscriber",
        json.name,
        json.email
    );
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, name, email, subscribed_at)
        VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        json.name,
        json.email,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .await
    {
        Ok(_) => {
            log::info!("request_id {request_id} New subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("request_id {request_id} Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
