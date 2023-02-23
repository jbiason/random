use std::thread;

fn main() {
    let concrete = String::from("Hello!");

    let ref1 = &concrete;
    let t1 = thread::spawn(move || {
        for i in 0..5 {
            println!("Thread1: {} - {}", i, ref1);
        }
    });

    let ref2 = &concrete;
    let t2 = thread::spawn(move || {
        for i in 5..10 {
            println!("Thread2: {} - {}", i, ref2);
        }
    });

    t1.join();
    t2.join();
}
