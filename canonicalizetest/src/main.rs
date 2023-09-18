use std::path::Path;

fn main() {
    let path = Path::new(".");
    println!("{:?}", path.canonicalize());

    let path = Path::new(".").join("..");
    println!("{:?}", path.canonicalize());

    let path = Path::new(".").join("not-here");
    println!("{:?}", path.canonicalize());
}
