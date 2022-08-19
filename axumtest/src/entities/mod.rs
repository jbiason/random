//! Entities used in the system.

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
