use std::env;
use std::time::Instant;
use std::{fmt::Display, time::Duration};

use chrono::Local;
use reqwest::{Client, Response};
use serde_json::Value;
use tokio::sync::mpsc::{channel, Sender};

struct TaskId(u32);

impl Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Task {:0pad$}]", self.0.to_string(), pad = 3)
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // number of tasks to create
    let n = args[1].parse::<u32>().unwrap();

    let (send, mut recv) = channel::<String>(n as usize);
    let start = Instant::now();

    println!("{} Starting {} tasks", now(), n);
    for i in 0..n as u32 {
        // println!("{} Creating task {}", now(), i);
        let i = TaskId(i);
        tokio::spawn(send_request(i, send.clone()));
    }

    // Wait for the tasks to finish.
    //
    // We drop our sender first because the recv() call otherwise
    // sleeps forever.
    drop(send);

    // When every sender has gone out of scope, the recv call
    // will return with an error. We ignore the error.
    println!("{} Waiting for tasks to finish", now());

    while let Some(response_text) = recv.recv().await {
        // println!("{}", response_text);
    }

    println!("{} All tasks finished", now());
    println!("Done in {}ms", start.elapsed().as_millis());
    let _ = recv.recv().await;
}

async fn send_request(i: TaskId, result: Sender<String>) {
    println!("{} {} - Sending Request", now(), i);

    let url = format!("http://167.235.133.26:80/get?id={}", i);
    let timeout = Duration::from_millis(2000);
    let client = Client::builder()
        .timeout(timeout)
        .connect_timeout(timeout)
        .build()
        .unwrap();
    let response = client
        .get(url)
        .send()
        .await
        .expect("Request failed")
        .json::<Value>()
        .await;

    let response_text = response
        .unwrap()
        .get("args")
        .unwrap()
        .get("id")
        .unwrap()
        .to_string();

    println!("{} {} - Responded with: {}", now(), i, response_text);
    let _ = result.send(response_text).await;
}

fn now() -> String {
    Local::now().format("%H:%M:%S%.3f").to_string()
}

// struct Logs {
//     measurements: Vec<(String, Instant)>,
// }

// impl logs {
//     fn log_to(&mut self, log_name: &str) {
//         self.measurements.push((log_name, Instant::now()))
//     }
// }
