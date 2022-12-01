use opentelemetry::global;
use tracing_subscriber::{
    filter::LevelFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt, Layer,
};

pub fn init_tracing(name: &str, to_stdout: bool, to_remote: bool) {
    let subscriber = tracing_subscriber::registry();

    let remote_subscriber = if to_remote {
        // Allows to pass along context (i.e., trace IDs) across services
        global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
        let tracer = opentelemetry_jaeger::new_pipeline()
            .with_service_name(name)
            .install_simple()
            .unwrap();

        // Create a tracing layer with the configured tracer, only log info
        Some(
            tracing_opentelemetry::layer().with_tracer(tracer), // .with_filter(LevelFilter::INFO),
        )
    } else {
        None
    };

    let stdout_subscriber = if to_stdout {
        Some(fmt::Layer::default())
    } else {
        None
    };

    subscriber
        .with(remote_subscriber)
        .with(stdout_subscriber)
        .try_init()
        .unwrap();
}
