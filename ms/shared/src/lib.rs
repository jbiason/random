use chrono::offset::Utc;
use chrono::DateTime;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    received: DateTime<Utc>,
    message: String,
}

impl Message {
    pub fn new(message: &str) -> Self {
        Self {
            received: Utc::now(),
            message: message.into(),
        }
    }
}
