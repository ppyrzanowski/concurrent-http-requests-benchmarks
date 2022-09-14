use std::{env, error::Error, thread, time::Instant};
use ureq;

use opentelemetry::global;
use tracing::{info, span};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn init_tracing(name: &str) {
    // Allows you to pass along context (i.e., trace IDs) across services
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    // Sets up the machinery needed to export data to Jaeger
    // There are other OTel crates that provide pipelines for the vendors
    // mentioned earlier.
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name(name)
        .install_simple()
        .unwrap();

    // Create a tracing layer with the configured tracer
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // The SubscriberExt and SubscriberInitExt traits are needed to extend the
    // Registry to accept `opentelemetry (the OpenTelemetryLayer type).
    tracing_subscriber::registry()
        .with(opentelemetry)
        // Continue logging to stdout
        // .with(fmt::Layer::default())
        .try_init()
        .unwrap();
}

fn runner(id: u32) {
    info!("start request");
    let start = Instant::now();
    let res = ureq::get(&format!("http://127.0.0.1:5000/sleep/{id}"))
        .call()
        .unwrap();
    let elapsed = start.elapsed().as_millis();

    info!(
        status = res.status(),
        elapsed_ms = elapsed,
        result = res.into_string().unwrap(),
        "response"
    );
}

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let args: Vec<String> = env::args().collect();
    let n = args[1].parse::<u32>().unwrap();

    init_tracing("ureq_threads");

    let result = {
        let root = span!(tracing::Level::INFO, "app_start", work_units = 2);
        let _enter = root.enter();

        info!(n, "starting threads");

        let mut threads = Vec::new();

        let start = Instant::now();
        for id in 0..n {
            // info!("create task");
            let parent_span = root.clone();
            threads.push(thread::spawn(move || {
                span!(parent: &parent_span, tracing::Level::INFO, "request_task")
                    .in_scope(|| runner(id))
            }));
        }

        info!("waiting for threads to finish");
        for thread in threads {
            let _ = thread.join();
        }
        let requests_completed_in = start.elapsed().as_millis();

        info!("Done in {}ms", requests_completed_in);
        requests_completed_in
    };

    println!("{result}");
    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
