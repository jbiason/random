mod auth;
mod headers;

use std::sync::Arc;

use axum::middleware;
use axum::routing::get;
use headers::cipwd::CiPwd;
use headers::cirole::CiRole;
use headers::ciusr::CiUsr;

use axum::routing::Router;
use axum::TypedHeader;
use clap::Parser;
use dotenv::dotenv;
use mongodb::options::ClientOptions;
use mongodb::Client;
use mongodb::Database;

#[derive(Parser)]
struct Params {
    #[clap(env = "SRV_ADDR", default_value = "0.0.0.0:3000")]
    addr: String,

    #[clap(short, long, env = "MONGO_URI")]
    mongo_addr: String,
}

struct State {
    db: Database,
    collections: Vec<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let args = Params::parse();

    tracing::debug!("Connecting to mongo...");
    let mongo_options = ClientOptions::parse(&args.mongo_addr).await.unwrap();
    let client = Client::with_options(mongo_options).unwrap();
    let db = client.database("helyxTestDatabase");
    let collections = db.list_collection_names(None).await.unwrap();
    tracing::debug!("Collections = {:?}", &collections);

    let state = Arc::new(State { db, collections });

    let app = Router::new().route("/", get(index)).route(
        "/collections",
        get({
            let shared_state = Arc::clone(&state);
            move || get_collections(Arc::clone(&shared_state))
        })
        .route_layer(middleware::from_fn(move |req, next| {
            auth::ci_auth(req, next, "A", "A", "A")
        })),
    );

    tracing::info!(args.addr, "Server listening in");
    axum::Server::bind(&args.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index(
    TypedHeader(usr): TypedHeader<CiUsr>,
    TypedHeader(_pwd): TypedHeader<CiPwd>,
    TypedHeader(_role): TypedHeader<CiRole>,
) -> String {
    format!("Hellow {}", usr)
}

async fn get_collections(state: Arc<State>) -> String {
    format!("Collections")
}
