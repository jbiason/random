use clap::Parser;
use sqlx::postgres::PgListener;
use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::Postgres;
use sqlx::Pool;

async fn connect(url: &str) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(3)
        .connect(url)
        .await
        .expect("Failed to connect to the database")
}

async fn create_listener(channel: &str, url: &str) {
    let mut listener = PgListener::connect(&url)
        .await
        .expect("Failed to connect listener {channel}");
    listener
        .listen(channel)
        .await
        .expect("Failed to listen to channel {channel}");

    log::info!("Listener on {channel} ready...");
    loop {
        let notification = listener
            .recv()
            .await
            .expect("Failed to receive data from channel {channel}");
        log::info!("Notification: {:?}", notification);
    }
}

#[derive(Parser)]
#[clap(version, about)]
struct Args {
    /// URL for the database connection.
    #[clap(short, long, env = "DATABASE_URL")]
    database: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();
    let pool = connect(&args.database).await;
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Migration failure");

    let listener_type_1 = create_listener("type_1", &args.database);
    let listener_type_2 = create_listener("type_2", &args.database);
    let listener_type_3 = create_listener("type_3", &args.database);

    tokio::join!(listener_type_1, listener_type_2, listener_type_3);
}
