fn main() {
    let phrase = std::env::args().skip(1).collect::<Vec<String>>().join(" ");
    let mapper = [
        Box::new(|x: char| x.to_uppercase()),
        Box::new(|x: char| x.to_lowercase()),
    ]
    .iter()
    .cycle();
    println!("{}", phrase.chars().map(mapper.next()).collect().join(""));
}
