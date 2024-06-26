use std::path::Path;

use tracing::level_filters::LevelFilter;
use tracing_subscriber::{prelude::*, EnvFilter};

fn main() {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::OFF.into())
        .with_env_var("LOG")
        .from_env()
        .unwrap();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(env_filter)
        .init();
    println!("Hello, world!");

    let a = 2;
    let p = Path::new("pilulito");

    tracing::debug!(a);
    tracing::info!(?p);

    tracing::error!("No!");
}
