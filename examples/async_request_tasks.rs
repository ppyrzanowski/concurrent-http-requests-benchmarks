use std::env;
use std::time::Instant;
use std::{fmt::Display, time::Duration};

use chrono::Local;
use reqwest::Client;
use serde_json::Value;
use tabled::builder::Builder;
use tabled::object::Segment;
use tabled::{Alignment, Modify, Style};
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

    let (send, mut recv) = channel::<Logs>(n as usize);
    let start = Instant::now();

    println!("{} Starting {} tasks", now(), n);
    for i in 0..n as u32 {
        // println!("{} Creating task {}", now(), i);
        let i = TaskId(i);
        let logs = Logs::new();
        tokio::spawn(send_request(i, send.clone(), logs));
    }

    // Wait for the tasks to finish.
    //
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

async fn send_request(i: TaskId, result: Sender<Logs>, mut logs: Logs) {
    println!("{} {} - Sending Request", now(), i);

    let url = format!("http://167.235.133.26:80/get?id={}", i);
    let timeout = Duration::from_millis(2000);
    let client = Client::builder()
        .timeout(timeout)
        .connect_timeout(timeout)
        .build()
        .unwrap();
    logs.add("client created");

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

fn now() -> String {
    Local::now().format("%H:%M:%S%.3f").to_string()
}

struct Logs {
    measurements: Vec<(String, Instant)>,
}

impl Logs {
    fn new() -> Self {
        let mut log = Logs {
            measurements: Vec::new(),
        };
        log.add("init");
        log
    }

    fn add(&mut self, log_name: &str) {
        //! Add new event
        self.measurements
            .push((log_name.to_string(), Instant::now()))
    }

    fn get_cols(&self) -> Vec<String> {
        self.measurements
            .iter()
            .map(|item| item.0.to_string())
            .collect()
    }

    fn get_vals(&self, start: Instant) -> Vec<u128> {
        self.measurements
            .iter()
            .map(|item| item.1.duration_since(start).as_millis())
            .collect()
    }
}

fn print_logs_table(logs_vec: Vec<Logs>, start_time: Instant) {
    let mut builder = Builder::default();
    let mut columns: Option<Vec<String>> = None;
    for logs in logs_vec {
        if columns.is_none() {
            columns = Some(logs.get_cols());
        }
        let values = logs.get_vals(start_time);
        builder.add_record(values);
    }

    match columns {
        Some(columns) => {
            builder.set_columns(columns);

            let table = builder
                .build()
                .with(Style::psql())
                .with(Modify::new(Segment::all()).with(Alignment::right()));
            println!("{}", table);
        }
        None => {
            println!("At least one Logs struct is required to print a table.");
        }
    }
}

// impl Display for Logs {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{}\n{}",
//             self.get_cols().join(" | "),
//             self.get_vals().join(" | ")
//         )
//     }
// }
