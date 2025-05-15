use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use std::net::SocketAddr;

#[tokio::main(flavor = "multi_thread", worker_threads = 12)]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    println!("WebSocket server running at ws://{}", addr);

    while let Ok((stream, socket_addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, socket_addr));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream, addr: SocketAddr) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("Failed to accept connection from {}: {}", addr, e);
            return;
        }
    };

    // Minimal logging — comment out or remove this line if not needed
    // println!("New connection from {}", addr);

    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(_) => {
                // Respond with static message "Hello World"
                if let Err(_) = write.send("Hello World".into()).await {
                    break;
                }
            }
            Err(_) => break,
        }
    }

    // println!("Connection with {} closed", addr);
}
