use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let self_tx = tx.clone();

    let consumer = thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            if msg == 0 {
                println!("Quit");
                break;
            }

            println!("Message: {}", msg);
        }
    });

    let producer = thread::spawn(move || {
        for i in 1..12 {
            tx.send(i);
        }
        tx.send(0);
    });

    println!("Waiting producer...");
    producer.join();
    println!("Waiting consumer...");
    consumer.join();

}
