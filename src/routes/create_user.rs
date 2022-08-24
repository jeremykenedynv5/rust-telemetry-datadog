
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    name : String,
    email: String
}

#[tracing::instrument(
    name = "Adding a new user details",
    skip(form, connection_pool),
    fields(
        user_name = %form.name,
        user_email = %form.email
    )
)]
pub async fn create_user(form: web::Form<FormData>, connection_pool: web::Data<PgPool>) -> HttpResponse {
    tracing::info!("Creating User: {}", form.name);
    match insert_user(&connection_pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new user details in the database",
    skip(form, pool)
)]
pub async fn insert_user(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO userinfo (id, email, name, created_at)
        VALUES ($1, $2, $3, $4)
        "#,
        uuid::Uuid::new_v4(),
        form.email,
        form.name,
        chrono::Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the ? to return early
        // if the function failed, returning a sqlx::Error;
    })?;
    Ok(())
}
