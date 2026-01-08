use actix_web::{HttpResponse, web};
use chrono::Utc;
use log::info;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscibe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    log::info!("request_id {} - Adding '{}' '{}' as a new subscriber", request_id, form.email, form.name);
    log::info!("request_id {} - Saving new subsciber details in the database", request_id);
    match sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at) 
    VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            log::info!("request id {} - New subsciber details have been saved", request_id);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            log::error!("request id {} - Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        },
    }
}
