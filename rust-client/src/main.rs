mod implementations;
mod utils;

use crate::implementations::{hyper_tokio, ureq_threads};
use crate::utils::logging::init_tracing;
use clap::{Parser, ValueEnum};
use tracing::info;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Implementation {
    UreqThreads,
    HyperTokio,
}

#[derive(Parser)]
struct Args {
    /// What implementation to execute
    #[arg(value_enum)]
    implementation: Implementation,

    /// Number of concurrent requests to send
    #[clap(value_parser)]
    n: i32,

    /// Print to stdout
    #[clap(long, value_parser)]
    trace_stdout: bool,

    /// Send tracing to jaeger at 1.0.0.127:5000
    #[clap(long, value_parser)]
    trace_remote: bool,
}

fn main() {
    let args: Args = Args::parse();
    let n = args.n;

    // Enable logging if needed
    if args.trace_stdout || args.trace_remote {
        init_tracing(
            &format!("{:#?}", args.implementation),
            args.trace_stdout,
            args.trace_remote,
        );
    }

    tracing::info_span!("app_start").in_scope(|| {
        // Run implementation
        let execution_time = match args.implementation {
            Implementation::UreqThreads => ureq_threads::scedule(n),
            Implementation::HyperTokio => hyper_tokio::scedule(n),
        };
        info!("Done in {}ms", execution_time);

        println!("{execution_time}");
    });

    // Send all buffered messages to remote collector
    opentelemetry::global::shutdown_tracer_provider();
}
