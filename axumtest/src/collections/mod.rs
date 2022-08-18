//! Deals specifically with collections.

use std::sync::Arc;

use axum::middleware;
use axum::routing::get;
use axum::Json;
use axum::Router;

use crate::{auth, State};

/// Build the routes for the collections resource.
pub fn router(state: Arc<State>, ci_usr: String, ci_pwd: String, ci_role: String) -> Router {
    Router::new().route(
        "/collections",
        get({
            let shared_state = Arc::clone(&state);
            move || get_collections(Arc::clone(&shared_state))
        })
        .route_layer(middleware::from_fn(move |req, next| {
            let ci_usr = ci_usr.clone();
            let ci_pwd = ci_pwd.clone();
            let ci_role = ci_role.clone();

            auth::ci_auth(req, next, ci_usr, ci_pwd, ci_role)
        })),
    )
}

/// The response for retrieving the list of collections.
#[derive(serde::Serialize)]
struct Collections {
    status: String,
    collections: Vec<String>,
}

/// Return the list of collections.
async fn get_collections(state: Arc<State>) -> Json<Collections> {
    let collections = state.db.list_collection_names(None).await.unwrap();
    Json(Collections {
        status: "OK".into(),
        collections,
    })
}
