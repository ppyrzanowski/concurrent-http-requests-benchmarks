mod implementations;
mod utils;

use crate::implementations::threads_ureq;
use crate::utils::logging::init_tracing;
use clap::{Parser, ValueEnum};
use tracing::{info, span};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Implementation {
    ThreadsUreq,
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

    // enable logging if needed
    if args.trace_stdout || args.trace_remote {
        init_tracing(
            &format!("{:#?}", args.implementation),
            args.trace_stdout,
            args.trace_remote,
        );
    }

    let root_span = span!(tracing::Level::INFO, "app_start");
    let result = root_span.in_scope(|| {
        // run implementation
        let execution_time = match args.implementation {
            Implementation::ThreadsUreq => threads_ureq::scedule(n, &root_span),
        };
        info!("Done in {}ms", execution_time);
        execution_time
    });

    println!("{result}");
    // Send all buffered messages to remote collector
    opentelemetry::global::shutdown_tracer_provider();
}
