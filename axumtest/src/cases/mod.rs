//! Deal with cases.

use std::sync::Arc;

use axum::extract::Path;
use axum::middleware;
use axum::routing::get;
use axum::Router;
use futures::stream::TryStreamExt;
use futures::StreamExt;
use mongodb::bson::doc;
use mongodb::bson::Document;
use mongodb::options::FindOptions;

use crate::auth;
use crate::State;

/// Build the routes for the cases resource.
pub fn router(state: Arc<State>, ci_usr: String, ci_pwd: String, ci_role: String) -> Router {
    Router::new().route(
        "/collections/:collname/cases",
        get({
            let shared_state = Arc::clone(&state);
            move |path| all_cases_on_collection(path, shared_state)
        })
        .route_layer(middleware::from_fn(move |req, next| {
            let ci_usr = ci_usr.clone();
            let ci_pwd = ci_pwd.clone();
            let ci_role = ci_role.clone();

            auth::ci_auth(req, next, ci_usr, ci_pwd, ci_role)
        })),
    )
}

async fn all_cases_on_collection(Path(collection): Path<String>, state: Arc<State>) -> String {
    let collection = state.db.collection::<Document>(&collection);
    let options = FindOptions::builder()
        .projection(doc! { "caseID": 1 })
        .build();
    let cursor = collection.find(None, options).await.unwrap();

    let result = cursor
        .map(|r| r.unwrap().get_str("caseID").unwrap().to_lowercase())
        .collect::<Vec<String>>()
        .await;
    // while let Some(record) = cursor.try_next().await.unwrap() {
    //     tracing::debug!("{:?}", record.get_str("caseID").unwrap());
    // }

    tracing::debug!("Cases: {:?}", result);

    format!("Cases!")
}
