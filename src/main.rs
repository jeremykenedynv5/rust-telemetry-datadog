// Copyright © 2022 Yokesh Thirumoorthi
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// CREDITS
// Project: https://github.com/LukeMathWalker/tracing-actix-web/tree/main/examples/opentelemetry
// Copyright (c) 2022 LukeMathWalker
// License (MIT) https://github.com/LukeMathWalker/tracing-actix-web/blob/main/LICENSE-MIT

use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use opentelemetry::{
    global, runtime::TokioCurrentThread, sdk::propagation::TraceContextPropagator,
};
use std::io;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

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
async fn create_user(form: web::Form<FormData>, connection_pool: web::Data<PgPool>) -> HttpResponse {
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

fn init_telemetry() {
    let app_name = "tracing-actix-datadog";

    // Start a new Jaeger trace pipeline.
    // Spans are exported in batch - recommended setup for a production application.
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = opentelemetry_datadog::new_pipeline()
    // let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name(app_name)
        .install_batch(TokioCurrentThread)
        .expect("Failed to install OpenTelemetry tracer.");

    // Filter based on level - trace, debug, info, warn, error
    // Tunable via `RUST_LOG` env variable
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));
    // Create a `tracing` layer using the Jaeger tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    // Create a `tracing` layer to emit spans as structured logs to stdout
    let formatting_layer = BunyanFormattingLayer::new(app_name.into(), std::io::stdout);
    // Combined them all together in a `tracing` subscriber
    let subscriber = Registry::default()
        .with(env_filter)
        .with(telemetry)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to install `tracing` subscriber.")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    init_telemetry();
    
    // TODO: db connection string should be read from some env or config file
    let db_connection_string = "postgres://postgres:password@127.0.0.1:5432/userdb";

    let db_pool = PgPool::connect(db_connection_string).await.expect("Error connecting to database");
    
    // wrap the connection in a smart pointer
    let db_pool = web::Data::new(db_pool);

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/create_user", web::post().to(create_user))
            // Get a pointer copy and attach it to the application state
            .app_data(db_pool.clone())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;

    // Ensure all spans have been shipped to Jaeger.
    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
