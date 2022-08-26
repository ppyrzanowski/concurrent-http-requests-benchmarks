use std::{
    thread,
    time::{Duration, Instant},
};

use chrono::Local;
use reqwest::blocking::Client;
use scheduled_thread_pool::ScheduledThreadPool;

fn now() -> String {
    Local::now().format("%H:%M:%S%.3f").to_string()
}

fn print(id: u32, text: &str) {
    println!("{} ( {:0pad$} ) - {}", now(), id, text, pad = 4);
}

fn run(id: u32) {
    let start = Instant::now();
    let client = Client::new();
    print(id, "Start request");
    let response_log = match client.get(format!("http://0.0.0.0:8080/{id}")).send() {
        Ok(res) => {
            format!("Response {}", res.status())
        }
        Err(err) => {
            format!("Request failed: {}", err.to_string())
        }
    };
    let elapsed = start.elapsed();
    print(id, &response_log);
}

fn main() {
    let t_pool = ScheduledThreadPool::new(1000);

    let mut jobs = Vec::new();

    let start_time = Duration::from_secs(1);

    for id in 0..200 {
        print(id, "Create task");
        jobs.push(t_pool.execute_after(start_time, move || run(id)));
    }
    thread::sleep(Duration::from_secs(30));
}
