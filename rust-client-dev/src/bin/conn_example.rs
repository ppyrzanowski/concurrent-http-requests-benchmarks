use http::{Request, StatusCode};
use hyper::{client::conn, Body};
use tokio::net::TcpStream;
use tower::ServiceExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host= "127.0.0.1:5000";
    let uri = "/sleep/2";
    let target_stream = TcpStream::connect(host).await?;

    let (mut request_sender, connection) = conn::handshake(target_stream).await?;

    // spawn a task to poll the connection and drive the HTTP state
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Error in connection: {}", e);
        }
    });

    let request = Request::builder()
        // We need to manually add the host header because SendRequest does not
        .header("Host",host)
        .uri(uri)
        .method("GET")
        .body(Body::from(""))?;
    let response = request_sender.send_request(request).await?;
    assert!(response.status() == StatusCode::OK);

    // To send via the same connection again, it may not work as it may not be ready,
    // so we have to wait until the request_sender becomes ready.
    request_sender.ready().await?;
    let request = Request::builder()
        .header("Host", host)
        .uri(uri)
        .method("GET")
        .body(Body::from(""))?;
    let response = request_sender.send_request(request).await?;
    assert!(response.status() == StatusCode::OK);
    Ok(())
}

