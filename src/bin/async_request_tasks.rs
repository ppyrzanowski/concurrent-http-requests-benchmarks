use async_vs_threads::task::TaskId;
use async_vs_threads::timeline::*;
use reqwest::Client;
use serde_json::Value;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::mpsc::{channel, Sender};
use tokio::sync::Barrier;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // number of tasks to create
    let n = args[1].parse::<u32>().unwrap();

    let (send, mut recv) = channel::<Timeline>(n as usize);
    let start = Instant::now();

    let barrier = Arc::new(Barrier::new((n / 2) as usize));

    println!("{} Starting {} tasks", now(), n);
    for i in 0..n as u32 {
        // println!("{} Creating task {}", now(), i);
        let i = TaskId(i);
        let logs = Timeline::new();
        tokio::spawn(send_request(i, send.clone(), logs, barrier.clone()));
    }

    // Wait for the tasks to finish.
    // We drop our sender first because the recv() call otherwise
    // sleeps forever.
    drop(send);

    // When every sender has gone out of scope, the recv call
    // will return with an error. We ignore the error.
    println!("{} Waiting for tasks to finish", now());

    let mut logs = Vec::new();
    while let Some(log) = recv.recv().await {
        logs.push(log);
    }

    println!("{} All tasks finished", now());
    println!("Done in {}ms", start.elapsed().as_millis());

    print_logs_table(logs, start);
}

async fn send_request(i: TaskId, result: Sender<Timeline>, mut logs: Timeline, b: Arc<Barrier>) {
    println!("{} {} - Sending Request", now(), i);

    let url = format!("http://167.235.133.26:80/get?id={}", i);
    let timeout = Duration::from_millis(2000);
    let client = Client::builder()
        .timeout(timeout)
        .connect_timeout(timeout)
        .build()
        .unwrap();
    logs.add("client created");
    b.wait().await;
    logs.add("barrier released");

    let response = client
        .get(url)
        .send()
        .await
        .expect("Request failed")
        .json::<Value>()
        .await;
    logs.add("got response");

    let response_text = response
        .unwrap()
        .get("args")
        .unwrap()
        .get("id")
        .unwrap()
        .to_string();

    println!("{} {} - Responded with: {}", now(), i, response_text);
    logs.add("drop task");
    let _ = result.send(logs).await;
}
