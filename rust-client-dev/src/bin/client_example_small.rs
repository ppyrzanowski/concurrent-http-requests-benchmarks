// https://docs.rs/hyper/latest/hyper/client/index.html
// Small exmaple using `Client` to fetch url
use hyper::{body::HttpBody as _, Client, Uri};
extern crate pretty_env_logger;
#[macro_use] extern crate log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    let client = Client::new();
    let uri = Uri::from_static("http://127.0.0.1:5000/sleep/2");

    let mut handlers = Vec::new();

    for _i in 1..3 {
        let client = client.clone();
        let uri = uri.clone();
        let handler = tokio::spawn(async move {
            let res = client.get(uri.clone()).await.unwrap();

            // And then, if the request gets a response...
            println!("status: {}", res.status());

            // Concatenate the body stream into a single buffer...
            let buf = hyper::body::to_bytes(res).await.unwrap();

            println!("body: {:?}", buf);
        });
        handlers.push(handler);
    }

    for handler in handlers {
        handler.await;
    }

    Ok(())
}
