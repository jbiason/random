use std::{sync::Arc, thread};

use sqlx::{sqlite::SqlitePoolOptions, Sqlite, SqlitePool};
use tokio::runtime::Runtime;

struct Connector {
    runtime: Runtime,
    pool: SqlitePool,
}

impl Connector {
    pub fn new() -> Self {
        let rt = Runtime::new().unwrap();
        let conn = rt.block_on(async {
            SqlitePoolOptions::new()
                .max_connections(1)
                .connect(":memory:")
                .await
                .expect("Failed to connect to memory")
        });
        Self {
            runtime: rt,
            pool: conn,
        }
    }

    pub fn query(&self) -> i64 {
        self.runtime.block_on(async {
            let result: (i64,) = sqlx::query_as("SELECT 1")
                .fetch_one(&self.pool)
                .await
                .unwrap();
            result.0
        })
    }
}

fn main() {
    let connector = Connector::new();
    let shared = Arc::new(connector);

    let internal1 = shared.clone();
    let thread1 = thread::spawn(move || {
        for _ in 0..5 {
            println!("Thread 1: {}", internal1.query());
        }
    });
    let thread2 = thread::spawn(move || {
        for _ in 0..5 {
            println!("Thread 2: {}", shared.query());
        }
    });
    thread1.join().expect("Failed waiting for thread 1");
    thread2.join().expect("Failed waiting for thread 2");
}
