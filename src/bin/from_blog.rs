use async_vs_threads::logging::init;
use futures;
use reqwest;
use std::{env, time::Instant};
use tracing::info;

#[tokio::main]
async fn main() {
    init("from_blog");
    let args: Vec<String> = env::args().collect();
    let n = args[1].parse::<u32>().unwrap();

    let start = Instant::now();

    let handles: Vec<_> = (0..n)
        .map(|i| {
            let url = format!("http://127.0.0.1:5000/sleep/{i}");
            let task = tokio::task::spawn(async move {
                let result = reqwest::get(url).await.unwrap().text().await.unwrap();
                info!(result);
                result
            });
            info!(i, "Task created");
            task
        })
        .collect();

    info!("Waiting for tasks...");
    let _results = futures::future::join_all(handles).await;
    let elapsed = start.elapsed().as_micros();

    // info!(
    println!(
        "All tasks finished. Done in {}ms, {}us",
        elapsed as f64 / 1000.0,
        elapsed
    );
}
