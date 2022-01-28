use std::env;
use std::path::Path;

fn main() {
    let execution_name = env::args().nth(0).unwrap_or("???".to_string());
    let parser = Path::new(&execution_name)
        .file_name()
        .map(|x| x.to_str())
        .flatten()
        .unwrap_or("???");
    println!("I am {}", execution_name);
    println!("And the name is {}", parser);
}
