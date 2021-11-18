use chrono::offset::Utc;
use chrono::DateTime;
use serde::Serialize;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::sync::broadcast;
use tokio::time::sleep;
use tokio::time::Duration;

#[derive(Debug, Clone, Serialize)]
struct Message {
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

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init();
    let (tx, mut rx) = broadcast::channel::<Message>(30);

    tokio::join!(consumer(rx), producer(tx.clone()), listener(tx));
}

/// Produtor de conteúdo
async fn producer(sink: broadcast::Sender<Message>) {
    let messages = ["Hello world", "Goodbye Earth", "See you later alligator"];
    for message in messages.iter().cycle() {
        match sink.send(Message::new(message)) {
            Ok(_) => {} // log::debug!("Send: {}", message),
            Err(err) => log::error!("Error sending message: {:?}", err),
        }
        sleep(Duration::from_millis(500)).await;
    }
}

/// Aguada conexões de clientes
async fn listener(connector: broadcast::Sender<Message>) {
    let listener = TcpListener::bind("127.0.0.1:4435")
        .await
        .expect("Failed binding");
    loop {
        if let Ok((socket, addr)) = listener.accept().await {
            log::debug!("Got new client: {:?}", addr);
            let new_receiver = connector.subscribe();
            tokio::spawn(async move { worker(new_receiver, socket, addr).await });
        }
    }
}

/// Worker que envia as mensagens para os clientes
async fn worker(mut source: broadcast::Receiver<Message>, mut stream: TcpStream, addr: SocketAddr) {
    loop {
        let message = source.recv().await.expect("No message");
        log::debug!("Sending {:?} to {:?}", message, addr);
        stream
            .write_all(&bincode::serialize(&message).unwrap())
            .await
            .expect("Failed to send");
    }
}

/// Consumidor interno
/// Essa função existe porque o sender do broadcast precisa de pelo menos um listener.
async fn consumer(mut receiver: broadcast::Receiver<Message>) {
    loop {
        let _message = receiver.recv().await.expect("Invalid message");
        // log::debug!("Message: {:?}", message);
    }
}
