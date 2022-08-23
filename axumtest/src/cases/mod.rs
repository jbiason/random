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
use tokio::task::JoinHandle;

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
    let collection = Arc::new(state.db.collection::<Document>(&collection));
    let options = FindOptions::builder()
        .projection(doc! { "caseID": 1 })
        .build();
    let mut cursor = collection.find(None, options).await.unwrap();
    let mut calls = Vec::new();

    while let Some(record) = cursor.try_next().await.unwrap() {
        let name = record.get_str("caseID").unwrap();
        let filter = doc! { "caseID": name };
        let task_collection = Arc::clone(&collection);
        let task = tokio::task::spawn(async move {
            let record = task_collection.find_one(filter, None).await.unwrap();
            record
        });

        calls.push(task);
    }

    for handle in calls {
        let record = handle.await;
        tracing::debug!("Case: {:?}", record);
    }

    format!("Cases!")
}
