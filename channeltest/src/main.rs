use std::time::Duration;

use tokio::{sync::mpsc, time::sleep, task};

#[tokio::main(flavor = "current_thread")]
async fn do_tasks_complete_when_one_side_of_the_channel_closes() {
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

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (tx1, mut rx) = mpsc::channel(10);

    let tx2 = tx1.clone();
    let producer1 = task::spawn(async move {
        for i in 10..20 {
            println!("1: Produced {i}");
            tx1.send(i).await.expect("Failed to send t1");
            sleep(Duration::from_millis(i * 123)).await;
        }
        println!("==== P1 completed");
    });

    let producer2 = task::spawn(async move {
        for i in 0..10 {
            println!("2: Produced {i}");
            tx2.send(i).await.expect("Failed to send t2");
            sleep(Duration::from_millis(i * 231)).await;
        }
        println!("==== P2 completed");
    });

    let consumer = task::spawn(async move {
        let mut data = Vec::new();
        while let Some(i) = rx.recv().await {
            println!("Received {i}");
            data.push(i);
        }

        println!("**** Channels closed, processing results");
        println!("{:?}", data);
    });


    producer1.await.expect("P1 died");
    producer2.await.expect("P2 died");
    consumer.await.expect("Consumer died");
}
