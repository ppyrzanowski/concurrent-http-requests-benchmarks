use std::{thread, time::Instant};
use tracing::info;
use ureq;

fn runner(id: u32, url: &str) {
    info!("start request");
    let start = Instant::now();
    let _res = ureq::get(url).call().unwrap();
    let elapsed = start.elapsed().as_millis();

    info!(id, elapsed_ms = elapsed, "response");
}

pub fn scedule(n: i32) -> i32 {
    info!(n, "starting threads");

    let start = Instant::now();
    let mut threads = Vec::new();
    for id in 0..n as u32 {
        let url = format!("http://127.0.0.1:5000/sleep/{}", id);
        let request_span = tracing::info_span!("request_task");
        threads.push(thread::spawn(move || {
            let _entered = request_span.entered();
            runner(id, &url)
        }));
    }

    info!("waiting for threads to finish");
    for thread in threads {
        thread.join().unwrap();
    }

    let requests_completed_in = start.elapsed().as_millis() as i32;
    requests_completed_in
}
