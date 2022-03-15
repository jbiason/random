use tokio::sync::broadcast;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (tx, _) = broadcast::channel(1);

    for i in 0..30000 {
        let mut rx = tx.subscribe();
        let _consumer = tokio::spawn(async move {
            while let Ok(mess) = rx.recv().await {
                println!("Consumer {}: {}", i, mess);
            }
            println!("Consumer {} is done", i);
        });
    }

    let producer = tokio::spawn(async move {
        // let mut rng = 3;
        loop {
            tx.send(42).expect("Failed to send!");
            tokio::task::yield_now().await;
            // rng += 17 % 200;
            // let duration = tokio::time::Duration::from_millis(rng);
            // tokio::time::sleep(duration).await;
        }
    });

    let _ = producer.await;
}
