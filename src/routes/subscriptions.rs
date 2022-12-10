use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct SubscribePayload {
    name: String,
    email: String,
}

#[post("/subscribe")]
async fn subscribe(json: web::Json<SubscribePayload>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber",
        %request_id,
        subscriber_name = %json.name,
        subscriber_email = %json.email
    );
    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("Saving new subscriber details in the database",);
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
    .execute(pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!("request_id {request_id} New subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("request_id {request_id} Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
