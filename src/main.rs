// Copyright © 2022 Yokesh Thirumoorthi
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use actix_web::{web, App, HttpServer};
use opentelemetry::{global, sdk::propagation::TraceContextPropagator};
use rust_telemetry_datadog::routes::{create_user, health_check};
use rust_telemetry_datadog::telemetry::{get_subscriber, init_subscriber};
use sqlx::PgPool;
use std::io;
use tracing_actix_web::TracingLogger;

fn init_telemetry() {
    // Start a new trace pipeline.
    global::set_text_map_propagator(TraceContextPropagator::new());

    let subscriber = get_subscriber(
        "tracing-actix-datadog".into(),
        "info".into(),
        std::io::stdout,
    );

    init_subscriber(subscriber);
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    init_telemetry();

    // TODO: db connection string should be read from some env or config file
    let db_connection_string = "postgres://postgres:password@127.0.0.1:5432/userdb";

    let db_pool = PgPool::connect(db_connection_string)
        .await
        .expect("Error connecting to database");

    // wrap the connection in a smart pointer
    let db_pool = web::Data::new(db_pool);

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/", web::get().to(health_check))
            .route("/create_user", web::post().to(create_user))
            // Get a pointer copy and attach it to the application state
            .app_data(db_pool.clone())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;

    // Ensure all spans have been shipped to Otel.
    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
