use tokio::sync::mpsc;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (tx, mut rx) = mpsc::channel(2);
    if let Err(_) = tx.send(2).await {
        println!("Failed to send message");
    }
    if let Err(_) = tx.send(3).await {
        println!("Failed to send second message");
    }

    let response = rx.recv().await;
    println!("Response: {:?}", response);
}
