use std::thread;

fn main() {
    let (mut tx, rx) = spmc::channel();

    let mut handles = Vec::new();
    for n in 0..5 {
        let rx = rx.clone();
        handles.push(thread::spawn(move || {
            while let Ok(msg) = rx.recv() {
                // println!("Hello");
                println!("worker {} recvd: {}", n, msg);
            }
        }));
    }

    for i in 0..15 {
        tx.send(i).unwrap();
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
