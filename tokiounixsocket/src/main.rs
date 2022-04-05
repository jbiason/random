use std::io;
use std::path::Path;
use std::str::FromStr;

use bigdecimal::BigDecimal;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::Interest;
use tokio::net::UnixListener;
use tokio::net::UnixStream;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum Message {
    First(i64, String),
    Second(i64, Option<String>),
    Third(i64),
    Fourth(BigDecimal),
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let path = Path::new("./comm.socket");
    let _ = std::fs::remove_file(&path);

    // producer
    let path_prod = path.to_path_buf();
    let producer = tokio::spawn(async move {
        let listener = UnixListener::bind(&path_prod).unwrap();
        loop {
            match listener.accept().await {
                Ok((mut stream, _addr)) => {
                    println!("Client here");
                    tokio::spawn(async move { send_to_consumer(&mut stream).await })
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
        let mut socket = UnixStream::connect(&path_cons).await.unwrap();
        loop {
            let ready = socket.ready(Interest::READABLE).await.unwrap();
            if ready.is_readable() {
                let size = socket.read_u64().await.unwrap();
                println!("Content length: {}", size as usize);

                let mut record = vec![0u8; size as usize];
                loop {
                    match socket.read_exact(&mut record).await {
                        Ok(0) => {
                            tokio::task::yield_now().await;
                            continue;
                        }
                        Ok(n) => {
                            println!("Read {} bytes: {:?}", n, record);
                            let message: Result<Message, Box<bincode::ErrorKind>> =
                                bincode::deserialize(&record);
                            match message {
                                Ok(the_message) => println!("Message: {:?}", the_message),
                                Err(e) => println!("Deserialized message: {:?}", e),
                            }
                            break;
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            break;
                        }
                        Err(e) => {
                            println!("Read error: {:?}", e);
                            break;
                        }
                    }
                }
            }
        }
    });

    // and... WAIT!
    let _ = tokio::join!(producer, consumer);
}

async fn send_to_consumer(stream: &mut UnixStream) {
    let mut should_yield = false;
    let mut seq = 0;
    loop {
        let message = match seq % 5 {
            0 => Message::First(seq, format!("{} first", seq)),
            1 => Message::Second(seq, Some(format!("{} second", seq))),
            2 => Message::Third(seq),
            _ => Message::Fourth(BigDecimal::from_str(&format!("{}", seq)).unwrap()),
        };
        seq += 1;

        let encoded = bincode::serialize(&message).unwrap();
        let len = encoded.len();

        let ready = stream.ready(Interest::WRITABLE).await.unwrap();
        if ready.is_writable() {
            // Header, with the size
            stream.write_u64(len as u64).await.unwrap();

            // The content
            match stream.try_write(&encoded) {
                Ok(n) => println!("Wrote {} bytes: {:?}", n, encoded),
                Err(e) => println!("Write error: {:?}", e),
            }

            should_yield = !should_yield;
            if should_yield {
                tokio::task::yield_now().await;
            }
        }
    }
}
