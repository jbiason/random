mod beacon;

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::time::Duration;

use beacon::Beacon;
use crossbeam::channel::unbounded;
use rand::thread_rng;
use rand::Rng;

fn main() {
    let semaphore = Arc::new(Beacon::new(10));
    let (send, receive) = unbounded();

    let mut waits = Vec::new();
    for i in 0..3 {
        let thread_recv = receive.clone();
        let thread_semaphore = Arc::clone(&semaphore);
        waits.push(std::thread::spawn(move || {
            let me = i;
            let mut sum = 0;

            while let Ok(value) = thread_recv.recv() {
                println!("{me} Got value: {value}");

                thread_semaphore.lease(value);

                let duration = Duration::new(value as u64, 0);
                std::thread::sleep(duration);
                sum += value;

                thread_semaphore.release(value);
            }
            println!("{me} completed with a total of {sum}");
            sum
        }));
    }
    drop(receive);

    println!("Sending messages:");
    let mut rng = thread_rng();
    let mut produced = 0;
    for i in 0..12 {
        let value = rng.gen_range(1..11);
        println!("{i}: produced value {value}");
        send.send(value).expect("Failed to send number");
        produced += value;
    }
    drop(send); // breaks the sender
    println!("Produced a total of {produced}");

    let mut total_result = 0;
    for handle in waits {
        let thread_result = handle.join().unwrap();
        total_result += thread_result;
    }

    println!("Threads consumed {total_result}");
}
