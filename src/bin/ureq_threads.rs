use async_vs_threads::logging::init;
use std::env;
use std::{thread, time::Instant};
use tracing::info;
use ureq;

fn runner(id: u32) {
    info!("start request");
    let start = Instant::now();
    let res = ureq::get(&format!("http://127.0.0.1:5000/sleep/{id}"))
        .call()
        .unwrap();
    let elapsed = start.elapsed().as_millis();

    info!(
        "[{}] ({}ms): {}",
        res.status(),
        elapsed,
        res.into_string().unwrap()
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[1].parse::<u32>().unwrap();
    init("threads");

    info!(n, "starting threads");

    let mut threads = Vec::new();

    let start = Instant::now();
    for id in 0..n {
        info!("create task");
        threads.push(thread::spawn(move || runner(id)));
    }
    let threads_created_in = start.elapsed().as_millis();

    info!("waiting for threads to finish");
    for thread in threads {
        let _ = thread.join();
    }
    let requests_completed_in = start.elapsed().as_millis();

    info!("Done in {}ms", requests_completed_in);
    println!("{requests_completed_in}");

    opentelemetry::global::shutdown_tracer_provider();
}
