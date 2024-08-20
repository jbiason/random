use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Condvar, Mutex,
};

pub struct Beacon {
    lock: Mutex<AtomicUsize>,
    guard: Condvar,
}

impl Beacon {
    pub fn new(leases: usize) -> Self {
        Self {
            lock: Mutex::new(AtomicUsize::new(leases)),
            guard: Condvar::new(),
        }
    }

    pub fn lease(&self, leases: usize) {
        let mut control = self.lock.lock().unwrap();
        let mut current_leases = control.load(Ordering::Relaxed);
        println!("Need {leases} out of {current_leases}");
        while current_leases < leases {
            println!("Not enough, waiting...");
            control = self.guard.wait(control).unwrap();
            current_leases = control.load(Ordering::Relaxed);
            println!("Need {leases} our of {current_leases}");
        }

        control.fetch_sub(leases, Ordering::SeqCst);
    }

    pub fn release(&self, leases: usize) {
        println!("Trying to release {leases} leases");
        let control = self.lock.lock().unwrap();
        println!("Releasing {leases} leases");
        control.fetch_add(leases, Ordering::SeqCst);
        self.guard.notify_all();
    }
}
