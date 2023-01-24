fn main() {
    let args = std::env::args().collect::<Vec<String>>().join(" ");
    println!("{:?}", shlex::split(&args));
}
