//! The definition of an actor.

use tokio::sync::mpsc;
use tokio::task;

#[async_trait::async_trait]
pub trait Actor {
    /// The type of value that the actor will receive.
    type Input: Send;

    /// Actor execution.
    async fn process(&self) -> (task::JoinHandle<()>, mpsc::Sender<Self::Input>);
}
