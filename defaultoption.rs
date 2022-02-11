#[derive(Debug, Default)]
struct Example {
    exists: Option<u8>,
}

fn main() {
    let e = Example::default();
    println!("Herro: {:?}", e);
}
