use opentelemetry::sdk::trace::Config;
use opentelemetry::sdk::Resource;
use opentelemetry::{global, KeyValue};
use opentelemetry_jaeger::JaegerTraceRuntime;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing_subscriber::{EnvFilter, Layer};

pub fn init(name: &str) {
    let filter = EnvFilter::from_default_env();
    // let fmt_layer = tracing_subscriber::fmt::Layer::default()
    //     .compact()
    //     .with_filter(filter);

    // global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name(name)
        .with_trace_config(
            Config::default().with_resource(Resource::new(vec![KeyValue::new(
                "exporter",
                "otlp-jaeger",
            )])),
        )
        // .install_batch(opentelemetry::runtime::Tokio)
        .install_simple()
        .unwrap();
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    tracing_subscriber::Registry::default()
        .with(opentelemetry)
        // .with(fmt_layer)
        .try_init()
        .unwrap();
    println!("Initialized tracing");
}
