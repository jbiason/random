use zerocopy::AsBytes;
use zerocopy::Unaligned;

// Apparently, zerocopy doesn't know how to make a Vec as bytes (examples show
// the use of array, so I believe the problem is that a Vec doesn't have a
// defined, specific size).
#[derive(AsBytes, Unaligned)]
#[repr(C)]
struct Entry {
    directories: Vec<String>,
    files: Vec<String>,
}

fn main() {
    let command = std::env::args().nth(1).unwrap();
    match &command[..] {
        "fill" => fill_db(),
        "search" => search_db(),
    }
}

fn fill_db() {
    let db = sled::open("example.db").unwrap();
    let root = Entry {
        directories: vec!["usr".into(), "etc".into()],
        files: vec!["boot.img".into()],
    };
    let mut conv = vec![];
    conv.extend_from_slice(&root.as_bytes());
    db.insert("/", conv);
}
