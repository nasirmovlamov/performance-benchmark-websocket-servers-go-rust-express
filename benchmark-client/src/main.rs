use futures_util::{SinkExt, StreamExt};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::Barrier;
use tokio::task;
use tokio_tungstenite::connect_async;

const NUM_CLIENTS: usize = 100;
const NUM_MESSAGES: usize = 10;
const SERVER_URL: &str = "ws://localhost:8080/ws";

#[tokio::main(flavor = "multi_thread", worker_threads = 20)]
async fn main() {
    let barrier = Arc::new(Barrier::new(NUM_CLIENTS + 1));
    let all_latencies = Arc::new(Mutex::new(Vec::with_capacity(NUM_CLIENTS * NUM_MESSAGES)));
    let start = Instant::now();
    let mut handles = Vec::with_capacity(NUM_CLIENTS);

    for client_id in 0..NUM_CLIENTS {
        let barrier_clone = barrier.clone();
        let latencies_clone = all_latencies.clone();

        let handle = task::spawn(async move {
            // Connect using &str URL directly
            let (ws_stream, _) = connect_async(SERVER_URL)
                .await
                .expect("Failed to connect");

            let (mut write, mut read) = ws_stream.split();

            barrier_clone.wait().await; // Synchronize start

            for msg_idx in 0..NUM_MESSAGES {
                let payload = format!("Client {}: Msg {}", client_id, msg_idx);
                let sent_time = Instant::now();

                write.send(payload.into()).await.unwrap();

                if let Some(Ok(_msg)) = read.next().await {
                    let latency = sent_time.elapsed();
                    latencies_clone.lock().unwrap().push(latency);
                }
            }
        });

        handles.push(handle);
    }

    barrier.wait().await; // Start clock once all clients are ready

    for handle in handles {
        handle.await.unwrap();
    }

    let total_duration = start.elapsed();
    let latencies = all_latencies.lock().unwrap();

    if latencies.is_empty() {
        println!("No latency data collected.");
        return;
    }

    let total_msgs = latencies.len();
    let min = latencies.iter().min().unwrap();
    let max = latencies.iter().max().unwrap();
    let sum: Duration = latencies.iter().sum();
    let avg = sum / total_msgs as u32;

    println!("📊 Benchmark Results:");
    println!("Total duration: {:?}", total_duration);
    println!("Total messages: {}", total_msgs);
    println!("Min latency: {:?}", min);
    println!("Max latency: {:?}", max);
    println!("Avg latency: {:?}", avg);
}
