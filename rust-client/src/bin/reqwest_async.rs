use async_vs_threads::logging::init;
use async_vs_threads::task::TaskId;
use reqwest::Client;
use serde_json::Value;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::mpsc::{channel, Sender};
use tokio::sync::Barrier;
use tracing::debug;
use tracing::info;

#[tokio::main]
async fn main() {
    init("concurrent-requests");
    let args: Vec<String> = env::args().collect();

    // number of tasks to create
    let n = args[1].parse::<u32>().unwrap();

    let (send, mut recv) = channel::<()>(n as usize);
    let barrier = Arc::new(Barrier::new((n / 2) as usize));

    info!("Starting {} tasks", n);
    let start = Instant::now();

    for i in 0..n as u32 {
        let task_id = TaskId(i);
        tokio::spawn(send_request(task_id, send.clone(), barrier.clone()));
    }

    // Wait for the tasks to finish.
    // We drop our sender first because the recv() call otherwise
    // sleeps forever.
    drop(send);

    // When every sender has gone out of scope, the recv call
    // will return with an error. We ignore the error.
    info!("Waiting for tasks to finish");
    let _ = recv.recv().await;

    info!(
        "All tasks finished. Done in {}ms",
        start.elapsed().as_millis()
    );

    opentelemetry::global::shutdown_tracer_provider();
}

async fn send_request(i: TaskId, _sender: Sender<()>, b: Arc<Barrier>) {
    info!("sending request");

    // let url = format!("http://167.235.133.26:80/delay/5?id={}", i);
    let url = format!("http://127.0.0.1:5000/sleep/{}", &i.0);

    let timeout = Duration::from_millis(9000);
    // let client = Client::builder()
    //     .timeout(timeout)
    //     .connect_timeout(timeout)
    //     .build()
    //     .unwrap();
    // info!("client created");
    info!("waiting for barrier");
    b.wait().await;
    info!("barrier released");

    let response = reqwest::get(url).await.expect("Request failed");
    info!("got response");

    let status = response.status();

    let response_body = response.json::<Value>().await.unwrap();

    let response_text = response_body
        .get("args")
        .unwrap()
        .get("id")
        .unwrap()
        .to_string();

    // info! {%status, response_text, "Response"};
}
