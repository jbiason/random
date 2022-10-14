use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let self_tx = tx.clone();

    let consumer = thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            println!("Message: {}", msg);

            if msg > 1000 {
                // actually, we just need to drop self_tx, otherwise the consumer will keep waiting
                // for inputs from it, even when tx was already dropped when the producer ended.
                // the problem with a direct drop is that rustc can't see that it won't be used
                // anymore.
                break;
            } else if msg % 2 == 0 {
                if self_tx.send(msg * 2).is_err() {
                    println!("Failed to push new value to consumer");
                    break;
                };
            }
        }
    });

    let producer = thread::spawn(move || {
        for i in 1..12 {
            if tx.send(i).is_err() {
                println!("Failed to send {}, ending producer", i);
                break;
            }
        }
        // tx.send(0);
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
