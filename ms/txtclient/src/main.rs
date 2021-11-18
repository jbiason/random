use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

use shared::Message;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init();

    let mut stream = TcpStream::connect("127.0.0.1:4435")
        .await
        .expect("Failed to connect to server");
    loop {
        let mut buffer = [0; 1024];
        let bytes = stream.read(&mut buffer).await.expect("Failed to read data");
        let decoded: Message =
            bincode::deserialize(&buffer[..bytes]).expect("Failed to convert message");
        log::info!("Got {:?}", decoded);
    }
}
