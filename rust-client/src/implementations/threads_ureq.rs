use std::{thread, time::Instant};
use tracing::{info, span};
use ureq;

fn runner(id: u32) {
    info!("start request");
    let start = Instant::now();
    let res = ureq::get(&format!("http://127.0.0.1:5000/sleep/{id}"))
        .call()
        .unwrap();
    let elapsed = start.elapsed().as_millis();

    info!(
        id,
        status = res.status(),
        elapsed_ms = elapsed,
        result = res.into_string().unwrap(),
        "response"
    );
}

pub fn scedule(n: i32, parent_span: &tracing::Span) -> i32 {
    info!(n, "starting threads");

    let mut threads = Vec::new();

    let start = Instant::now();
    for id in 0..n as u32 {
        let parent_span = parent_span.clone();
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

    requests_completed_in as i32
}
