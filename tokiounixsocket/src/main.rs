use std::io;
use std::path::Path;

use tokio::io::Interest;
use tokio::net::UnixListener;
use tokio::net::UnixStream;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum Message {
    First(i64, String),
    Second(i64, String),
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let path = Path::new("./comm.socket");

    // producer
    let path_prod = path.to_path_buf();
    let producer = tokio::spawn(async move {
        let listener = UnixListener::bind(&path_prod).unwrap();
        loop {
            match listener.accept().await {
                Ok((stream, _addr)) => {
                    println!("Client here");
                    tokio::spawn(async move { send_to_consumer(&stream).await })
                }
                Err(e) => {
                    println!("Erro! {:?}", e);
                    break;
                }
            };
        }
    });

    // consumer
    let path_cons = path.to_path_buf();
    let consumer = tokio::spawn(async move {
        let socket = UnixStream::connect(&path_cons).await.unwrap();
        loop {
            let ready = socket.ready(Interest::READABLE).await.unwrap();
            if ready.is_readable() {
                let mut data = [0; 1024];
                match socket.try_read(&mut data) {
                    Ok(n) => {
                        println!("Read {} bytes", n);
                        let message: Result<Message, Box<bincode::ErrorKind>> =
                            bincode::deserialize(&data[..n]);
                        match message {
                            Ok(the_message) => println!("Message: {:?}", the_message),
                            Err(e) => println!("Deserialized message: {:?}", e),
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => {
                        println!("Error reading: {:?}", e);
                    }
                }
            }
        }
    });

    // and... WAIT!
    let _ = tokio::join!(producer, consumer);
}

async fn send_to_consumer(stream: &UnixStream) {
    let mut should_yield = false;
    let mut seq = 0;
    loop {
        let message = if should_yield {
            Message::First(seq, format!("{} first", seq))
        } else {
            Message::Second(seq, format!("{} second", seq))
        };
        seq += 1;

        let encoded = bincode::serialize(&message).unwrap();

        let ready = stream.ready(Interest::WRITABLE).await.unwrap();
        if ready.is_writable() {
            match stream.try_write(&encoded) {
                Ok(n) => println!("Wrote {} bytes: {:?}", n, encoded),
                Err(e) => println!("Error: {:?}", e),
            }

            should_yield = !should_yield;
            if should_yield {
                tokio::task::yield_now().await;
            }
        }
    }
}
