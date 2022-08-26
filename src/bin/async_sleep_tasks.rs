use std::fmt::Display;
use std::time::Instant;

use chrono::Local;
use tokio::sync::mpsc::{channel, Sender};
use tokio::time::{sleep, Duration};

struct TaskId(u32);

impl Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Task {:0pad$}]", self.0.to_string(), pad = 3)
    }
}

#[tokio::main]
async fn main() {
    let n = 1000;
    let sleep: u64 = 2000;
    let (send, mut recv) = channel(1);
    let start = Instant::now();

    println!("{} Starting {} tasks", now(), n);
    for i in 0..n {
        // println!("{} Creating task {}", now(), i);
        let i = TaskId(i);
        tokio::spawn(some_operation(i, sleep, send.clone()));
    }

    // Wait for the tasks to finish.
    //
    // We drop our sender first because the recv() call otherwise
    // sleeps forever.
    drop(send);

    // When every sender has gone out of scope, the recv call
    // will return with an error. We ignore the error.
    println!("{} Waiting for tasks to finish", now());
    let _ = recv.recv().await;
    println!("{} All tasks finished", now());
    println!("Done in {}ms", start.elapsed().as_millis() as u64 - sleep);
}

async fn some_operation(i: TaskId, duration: u64, _sender: Sender<()>) {
    // println!("{} {} sleeping for 2000ms", now(), i);
    sleep(Duration::from_millis(duration)).await;
    // println!("{} {} shutting down.", now(), i);
}

fn now() -> String {
    Local::now().format("%H:%M:%S%.3f").to_string()
}
