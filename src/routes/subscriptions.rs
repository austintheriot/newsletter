use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, Error, PgPool};
use tracing::Instrument;
use uuid::Uuid;

use crate::NewSubscriber;

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
        subscriber_name = %json.name,
        subscriber_email = %json.email
    )
)]
async fn subscribe(pool: web::Data<PgPool>, json: web::Json<SubscribePayload>) -> HttpResponse {
    let new_subscriber = match NewSubscriber::parse(&json.name, &json.email) {
        Ok(new_subscriber) => new_subscriber,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match insert_subscriber(pool.get_ref(), &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Inserts a subscriber into the database
#[tracing::instrument(skip(pool))]
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<PgQueryResult, Error> {
    let query_span = tracing::info_span!("Saving new subscriber details in the database",);
    let query_result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, name, email, subscribed_at)
        VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        new_subscriber.name.as_ref(),
        new_subscriber.email.as_ref(),
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
