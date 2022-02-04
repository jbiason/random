use tokio::sync::mpsc::channel;
use tokio::time::sleep;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = channel(100);
    let (tx2, mut rx2) = channel(100);

    tokio::spawn(async move {
        println!("I'm worker 1");
        let mut loop_id = 1;
        let sleep_time = Duration::from_millis(200);
        loop {
            let message = format!("This is loop 1-{loop_id}");
            tx1.send(message).await.unwrap();
            sleep(sleep_time).await;
            loop_id += 1;
        }
    });

    tokio::spawn(async move {
        println!("I'm worker 2");
        let mut loop_id = 1;
        let sleep_time = Duration::from_millis(300);
        loop {
            let message = format!("This is loop 2-{loop_id}");
            tx2.send(message).await.unwrap();
            sleep(sleep_time).await;
            loop_id += 1;
        }
    });

    // This is main
    loop {
        tokio::select! {
            Some(msg) = rx1.recv() => {
                println!("Worker 1 said \"{}\"", msg);
            }
            Some(msg) = rx2.recv() => {
                println!("Worker 2 said \"{}\"", msg);
            }
            else => { break }
        };
    }
}
