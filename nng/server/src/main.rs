use nanomsg::Protocol;
use nanomsg::Socket;

fn main() {
    println!("Starting server...");

    let mut socket = Socket::new(Protocol::Sub).unwrap();
}
