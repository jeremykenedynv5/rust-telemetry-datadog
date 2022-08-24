/// Copyright © 2022 Yokesh Thirumoorthi
/// [This program is licensed under the "MIT License"]
/// Please see the file LICENSE in the source
/// distribution of this software for license terms.

/// CREDITS
/// Project: https://github.com/LukeMathWalker/tracing-actix-web/tree/main/examples/opentelemetry
/// Copyright (c) 2022 LukeMathWalker
/// License (MIT) https://github.com/LukeMathWalker/tracing-actix-web/blob/main/LICENSE-MIT

use opentelemetry::runtime::TokioCurrentThread;
use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

/// There are some incompatibilities between datadog and OTel.
/// and see more details about it in the following links.
///      1. https://docs.rs/opentelemetry-datadog/latest/opentelemetry_datadog/#quirks
///      2. https://docs.datadoghq.com/tracing/other_telemetry/connect_logs_and_traces/opentelemetry
///      3. https://github.com/open-telemetry/opentelemetry-rust/issues/820
///      4. https://github.com/tokio-rs/tracing/issues/1531
/// In order to circumvent the above issues, we send the traces to
/// OTEL collector and use Datadog exporter to forword them to Datadog. More readings could be found in
/// this link - https://docs.datadoghq.com/tracing/trace_collection/open_standards/otel_collector_datadog_exporter/
/// First, create a OTLP exporter builder. Configure it as you need.
/// Ref: https://docs.rs/opentelemetry-otlp/latest/opentelemetry_otlp/
pub fn get_subscriber<Sink>(
    app_name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
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
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new(env_filter));
    
    // Create a `tracing` layer.
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    
    // Create a `tracing` layer to emit spans as structured logs to stdout
    let formatting_layer = BunyanFormattingLayer::new(app_name, sink);
    
    // Combined them all together in a `tracing` subscriber
    Registry::default()
        .with(env_filter)
        .with(telemetry)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

//// Register a subscriber as global default to proces span data.
///
//// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to install `tracing` subscriber");
}
