use std::time::Duration;

use tokio::{sync::mpsc, time::sleep, task};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (tx, mut rx) = mpsc::channel(10);

    let producer = task::spawn(async move {
        for i in 0..10 {
            println!("Produced {i}");
            tx.send(i).await.expect("Failed to send data");
            sleep(Duration::from_millis(i * 100)).await;
        }
        println!("=== Producer is done!")
    });

    let consumer = task::spawn(async move {
        while let Some(i) = rx.recv().await {
            println!("Received {i}");
            sleep(Duration::from_millis(i * 200)).await;
        }
    });

    producer.await.expect("Producer died");
    consumer.await.expect("Consumer died");
}
