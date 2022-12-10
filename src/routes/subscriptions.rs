use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, Error, PgPool};
use tracing::Instrument;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct SubscribePayload {
    name: String,
    email: String,
}

#[post("/subscribe")]
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(pool, json),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_name = %json.name,
        subscriber_email = %json.email
    )
)]
async fn subscribe(pool: web::Data<PgPool>, json: web::Json<SubscribePayload>) -> HttpResponse {
    match insert_subscriber(pool.get_ref(), &json).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Inserts a subscriber into the database
pub async fn insert_subscriber(
    pool: &PgPool,
    subscribe_payload: &SubscribePayload,
) -> Result<PgQueryResult, Error> {
    let query_span = tracing::info_span!("Saving new subscriber details in the database",);
    let query_result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, name, email, subscribed_at)
        VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        subscribe_payload.name,
        subscribe_payload.email,
        Utc::now()
    )
    .execute(pool)
    .instrument(query_span)
    .await
    .map_err(|e| {
        tracing::error!("failed to execute query: {:?}", e);
        e
    })?;
    Ok(query_result)
}
