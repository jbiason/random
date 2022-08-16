mod headers;

use axum::routing::get;
use axum::routing::Router;
use axum::TypedHeader;
use headers::cipwd::CiPwd;
use headers::cirole::CiRole;
use headers::ciusr::CiUsr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index(
    TypedHeader(usr): TypedHeader<CiUsr>,
    TypedHeader(pwd): TypedHeader<CiPwd>,
    TypedHeader(role): TypedHeader<CiRole>,
) -> String {
    format!("Hellow {}", usr)
}
