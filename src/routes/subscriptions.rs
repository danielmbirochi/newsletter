use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use tracing::Instrument;

#[derive(serde::Deserialize)]
pub struct SubscriptionFormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<SubscriptionFormData>,
    conn: web::Data<PgPool>,
) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding new subscriber.",
        %request_id,
        email = %form.email,
        name = %form.name
    );
    let _request_span_guard = request_span.enter();

    // We do not call `.enter` on query_span, `.instrument`
    // takes care of it at the right moments in the query future lifetime.
    let query_span = tracing::info_span!("Saving new subscriber to database");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, name, email, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.name,
        form.email,
        Utc::now()
    )
    .execute(conn.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            tracing::error!("Failed to execute query : {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

