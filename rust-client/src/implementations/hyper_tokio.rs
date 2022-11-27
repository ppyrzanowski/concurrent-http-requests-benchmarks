use hyper::{client::HttpConnector, Client, Uri};
use std::time::Instant;
use tokio::runtime::Runtime;
use tracing::{info, span, Instrument};

async fn runner(id: u32, client: Client<HttpConnector>, uri: Uri) {
    info!("start request");
    let start = Instant::now();
    let res = client.get(uri).await.unwrap();
    let elapsed = start.elapsed().as_millis();

    // println!("status: {}", res.status());

    // Concatenate the body stream into a single buffer
    // let buf = hyper::body::to_bytes(res).await.unwrap();

    // println!("body: {:?}", buf);

    info!(
        id,
        // status = res.status(),
        elapsed_ms = elapsed,
        // result = res.into_string().unwrap(),
        "response"
    );
}

pub fn scedule(n: i32, parent_span: &tracing::Span) -> i32 {
    info!(n, "starting threads");

    let start = Instant::now();

    let rt = Runtime::new().unwrap();

    rt.block_on(async move {
        let mut tasks = Vec::new();

        let client = Client::new();
        let uri = Uri::from_static("http://127.0.0.1:5000/sleep/2");

        for id in 0..n as u32 {
            let parent_span = parent_span.clone();
            let span = span!(parent: &parent_span, tracing::Level::INFO, "request_task");

            let client = client.clone();
            let uri = uri.clone();

            tasks.push(tokio::spawn(async move { runner(id, client, uri).await }).instrument(span));
        }

        info!("waiting for threads to finish");
        for task in tasks {
            let _ = task.await.unwrap();
        }
    });
    let requests_completed_in = start.elapsed().as_millis();

    requests_completed_in as i32
}
