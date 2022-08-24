// Copyright © 2022 Yokesh Thirumoorthi
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// CREDITS
// Project: https://github.com/LukeMathWalker/tracing-actix-web/tree/main/examples/opentelemetry
// Copyright (c) 2022 LukeMathWalker
// License (MIT) https://github.com/LukeMathWalker/tracing-actix-web/blob/main/LICENSE-MIT

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use opentelemetry::{
    global, runtime::TokioCurrentThread, sdk::propagation::TraceContextPropagator,
};
use std::io;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};
use rust_telemetry_datadog::routes::{ health_check, create_user };

fn init_telemetry() {
    let app_name = "tracing-actix-datadog";

    // Start a new trace pipeline.
    global::set_text_map_propagator(TraceContextPropagator::new());

    // There are some incompatibilities between datadog and OTel.
    // and see more details about it in the following links.
    //      1. https://docs.rs/opentelemetry-datadog/latest/opentelemetry_datadog/#quirks
    //      2. https://docs.datadoghq.com/tracing/other_telemetry/connect_logs_and_traces/opentelemetry
    //      3. https://github.com/open-telemetry/opentelemetry-rust/issues/820
    //      4. https://github.com/tokio-rs/tracing/issues/1531
    // In order to circumvent the above issues, we send the traces to
    // OTEL collector and use Datadog exporter to forword them to Datadog. More readings could be found in 
    // this link - https://docs.datadoghq.com/tracing/trace_collection/open_standards/otel_collector_datadog_exporter/

    // First, create a OTLP exporter builder. Configure it as you need.
    // Ref: https://docs.rs/opentelemetry-otlp/latest/opentelemetry_otlp/
    let otlp_exporter = opentelemetry_otlp::new_exporter().tonic();
    
    // Then pass it into pipeline builder
    // Spans are exported in batch - recommended setup for a production application.
    // TODO: Setup a service name for the tracer.
    let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(otlp_exporter) 
            .install_batch(TokioCurrentThread)
            .expect("Failed to install OpenTelemetry tracer.");

    // Filter based on level - trace, debug, info, warn, error
    // Tunable via `RUST_LOG` env variable
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));
    // Create a `tracing` layer.
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

