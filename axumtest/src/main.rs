mod header;

use axum::routing::get;
use axum::routing::Router;
use axum::TypedHeader;
use header::ciusr::CiUsr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index(TypedHeader(usr): TypedHeader<CiUsr>) -> String {
    format!("Hellow {}", usr)
}
