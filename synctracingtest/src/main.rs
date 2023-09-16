use std::path::Path;

use tracing_subscriber::prelude::*;

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    println!("Hello, world!");

    let a = 2;
    let p = Path::new("pilulito");

    tracing::debug!(a);
    tracing::info!(?p);
}
