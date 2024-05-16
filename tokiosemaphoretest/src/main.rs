use tokio::sync::Semaphore;

#[tokio::main]
async fn main() {
    let sem = Semaphore::new(4);

    let _p = sem.acquire_many(3).await;
    println!("P acquired: {_p:?}");

    // this locks :(
    let _q = sem.acquire_many(12).await;
    println!("12 acquired: {_q:?}");
}
