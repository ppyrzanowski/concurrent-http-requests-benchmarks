//! Lower-level client connection API.
//!
//! The types in this module are to provide a lower-level API based around a
//! single connection. Connecting to a host, pooling connections, and the like
//! are not handled at this level. This module provides the building blocks to
//! customize those things externally.
//!
//! If don't have need to manage connections yourself, consider using the
//! higher-level [Client](super) API.
//!
//! ## Example
//! A simple example that uses the `SendRequest` struct to talk HTTP over a Tokio TCP stream

use http::{Request, StatusCode};
use hyper::{client::conn, Body};
use tokio::net::TcpStream;
use tower::ServiceExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_stream = TcpStream::connect("127.0.0.1:5000").await?;

    let (mut request_sender, connection) = conn::handshake(target_stream).await?;

    // spawn a task to poll the connection and drive the HTTP state
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Error in connection: {}", e);
        }
    });

    // We need to manually add the host header because SendRequest does not
    let request = Request::builder()
        .header("Host", "127.0.0.1:5000")
        .method("GET")
        .uri("/sleep/2")
        .body(Body::from(""))?;
    let response = request_sender.send_request(request).await?;
    assert!(response.status() == StatusCode::OK);
    println!("{:#?}", response.headers());

    // To send via the same connection again, it may not work as it may not be ready,
    // so we have to wait until the request_sender becomes ready.
    request_sender.ready().await?;
    let request = Request::builder()
        .header("Host", "127.0.0.1:5000")
        .method("GET")
        .uri("/sleep/2")
        .body(Body::from(""))?;
    let response = request_sender.send_request(request).await?;
    assert!(response.status() == StatusCode::OK);
    Ok(())
}
