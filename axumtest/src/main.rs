mod auth;
mod cases;
mod collections;
mod entities;

use std::sync::Arc;

use axum::routing::get;
use axum::routing::Router;
use clap::Parser;
use dotenv::dotenv;
use mongodb::options::ClientOptions;
use mongodb::Client;

use crate::entities::Params;
use crate::entities::State;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let args = Params::parse();

    tracing::debug!("Connecting to mongo...");
    let mongo_options = ClientOptions::parse(&args.mongo_addr).await.unwrap();
    let client = Client::with_options(mongo_options).unwrap();
    let db = client.database("helyxTestDatabase");

    let state = Arc::new(State { db });

    let app = Router::new()
        .route("/", get(index))
        .merge(collections::router(
            state.clone(),
            args.ci_usr.clone(),
            args.ci_pwd.clone(),
            args.ci_role.clone(),
        ))
        .merge(cases::router(
            state.clone(),
            args.ci_usr.clone(),
            args.ci_pwd.clone(),
            args.ci_role.clone(),
        ));

    tracing::info!(args.addr, "Server listening in");
    axum::Server::bind(&args.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> String {
    format!("Hellow")
}
