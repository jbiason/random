mod auth;

use std::sync::Arc;

use axum::middleware;
use axum::routing::get;
use axum::Json;

use axum::routing::Router;
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

    #[clap(long, env = "CIUSR")]
    ci_usr: String,

    #[clap(long, env = "CIPWD")]
    ci_pwd: String,

    #[clap(long, env = "CIROLE")]
    ci_role: String,
}
struct State {
    db: Database,
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

    let state = Arc::new(State {
        db,
    });

    let app = Router::new().route("/", get(index)).route(
        "/collections",
        get({
            let shared_state = Arc::clone(&state);
            move || get_collections(Arc::clone(&shared_state))
        })
        .route_layer(middleware::from_fn(move |req, next| {
            let ci_usr = args.ci_usr.clone();
            let ci_pwd = args.ci_pwd.clone();
            let ci_role = args.ci_role.clone();

            auth::ci_auth(req, next, ci_usr, ci_pwd, ci_role)
        })),
    );

    tracing::info!(args.addr, "Server listening in");
    axum::Server::bind(&args.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> String {
    format!("Hellow")
}

async fn get_collections(state: Arc<State>) -> Json<Vec<String>> {
    let collections = state.db.list_collection_names(None).await.unwrap();
    Json(collections)
}
