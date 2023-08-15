//! Actor 1.

use tokio::{task, sync::mpsc};

use crate::actor::Actor;

pub struct Actor1 {}

impl Actor1 {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl Actor for Actor1 {
    type Input = ();

    async fn process(&self) -> (task::JoinHandle<()>, mpsc::Sender<Self::Input>) {
        let (tx, mut rx) = mpsc::channel::<Self::Input>(10);
        let task = task::spawn(async move {
            while let Some(data) = rx.recv().await {
                println!("{:?}", data);
            }
        });

        (task, tx)
    }
}
