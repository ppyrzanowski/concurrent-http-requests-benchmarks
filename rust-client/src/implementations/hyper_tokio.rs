use hyper::{client::HttpConnector, Client, Uri};
use std::{str::FromStr, time::Instant};
use tokio::runtime::Runtime;
use tracing::{info, Instrument};

async fn runner(id: u32, client: Client<HttpConnector>, uri: Uri) {
    info!("start request");
    let start = Instant::now();
    let _res = client.get(uri).await.unwrap();
    let elapsed = start.elapsed().as_millis();

    info!(id, elapsed_ms = elapsed, "response");

    // Concatenate the body stream into a single buffer
    // let buf = hyper::body::to_bytes(res).await.unwrap();
}

pub fn scedule(n: i32) -> i32 {
    info!(n, "starting async tasks");

    let start = Instant::now();

    let rt = Runtime::new().unwrap();
    info!("runtime created");
    rt.block_on(
        async move {
            let mut tasks = Vec::new();
            let client = Client::new();

            for id in 0..n as u32 {
                let client = client.clone();
                let url = Uri::from_str(&format!("http://127.0.0.1:5000/sleep/{}", id)).unwrap();

                let span = tracing::info_span!("request_task");
                let task = async move { runner(id, client, url).await };

                tasks.push(tokio::spawn(task.instrument(span)));
            }

            info!("waiting for tasks to finish");
            for task in tasks {
                task.await.unwrap();
            }
        }
        .instrument(tracing::info_span!("async_block")),
    );

    let requests_completed_in = start.elapsed().as_millis() as i32;
    requests_completed_in
}
