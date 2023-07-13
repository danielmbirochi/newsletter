use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use crate::domain::{NewSubscriber, SubscriberName};

#[derive(serde::Deserialize)]
pub struct SubscriptionFormData {
    email: String,
    name: String,
}


#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, conn),
    fields(
        email = %form.email,
        name = %form.name
    )
)]
pub async fn subscribe(
    form: web::Form<SubscriptionFormData>,
    conn: web::Data<PgPool>,
) -> HttpResponse {
    let subscriber_name = match SubscriberName::parse(form.0.name) {
        Ok(name) => name,
        Err(e) => return HttpResponse::BadRequest().body(e),
    };

    let new_subscriber = NewSubscriber {
        email: form.0.email.to_string(),
        name: subscriber_name,
    };
    match insert_subscriber(&conn, &new_subscriber).await {
        Ok(_) => {
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            tracing::error!("Failed to execute query : {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}


#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, conn)
)]
pub async fn insert_subscriber(
    conn: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, name, email, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_subscriber.name.as_ref(),
        new_subscriber.email,
        Utc::now()
    )
    .execute(conn)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query : {:?}", e);
        e
    })?;
    Ok(())
}