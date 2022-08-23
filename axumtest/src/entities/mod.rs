//! Entities used in the system.

use std::collections::HashMap;

use clap::Parser;
use mongodb::Database;

/// State shared between routes.
pub struct State {
    pub db: Database,
}

/// Command line options.
#[derive(Parser)]
pub struct Params {
    /// Address to listen for requests.
    #[clap(env = "SRV_ADDR", default_value = "0.0.0.0:3000")]
    pub addr: String,

    /// URI to connect to MongoDB.
    #[clap(short, long, env = "MONGO_URI")]
    pub mongo_addr: String,

    /// Validation for the X-CIUSR header.
    #[clap(long, env = "CIUSR")]
    pub ci_usr: String,

    /// Validation for the X-CIPWD header.
    #[clap(long, env = "CIPWD")]
    pub ci_pwd: String,

    /// Validation for the X-CIROLE header.
    #[clap(long, env = "CIROLE")]
    pub ci_role: String,
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    status: String,
    reason: String,
}

impl ErrorResponse {
    pub fn new(reason: &str) -> Self {
        Self {
            status: "ERR".into(),
            reason: reason.into(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Comparison {
    absolute_tolerance: f64,
    relative_tolerance: f64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Compare {
    file_to_test: String,
    entries_to_compare: HashMap<String, Comparison>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Case {
    #[serde(default)]
    files: Vec<String>,
    timeout: Option<u16>,
    #[serde(default)]
    run: Vec<String>,
    #[serde(default)]
    parallel: Vec<String>,
    procs: Option<u16>,

    #[serde(default)]
    tags: Vec<String>,
    version: Option<u16>,

    #[serde(default)]
    compare: HashMap<String, Compare>,
}
