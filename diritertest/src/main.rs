use std::path::Path;

use crate::diriter::DirIter;

mod diriter;

fn main() {
    let mut all_iter = DirIter::new(&Path::new(".")).unwrap();
    while let Some(path) = all_iter.next() {
        println!(">> {:?}", path);
    }

    println!("-----------------------");

    let all_iter = DirIter::new(&Path::new(".")).unwrap();
    for path in all_iter {
        println!(">>> {:?}", path);
    }

    println!("Hello, world!");
}
