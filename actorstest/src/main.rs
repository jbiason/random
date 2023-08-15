use actor::Actor;

mod actor;

mod actor_1;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let one = actor_1::Actor1::new();
    let (worker1, channel1) = one.process().await;
}
