use std::{thread, time::Instant};

use chrono::Local;
use reqwest::blocking::Client;

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
    let mut threads = Vec::new();

    for id in 0..32 {
        print(id, "Create task");
        threads.push(thread::spawn(move || run(id)));
    }

    for thread in threads {
        thread.join();
    }
}
