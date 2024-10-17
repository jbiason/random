use std::ffi::OsStr;
use std::path::Path;

use diriter::Accept;

use diriter::DirIter;

mod diriter;

struct AllFiles;
impl Accept for AllFiles {
    fn accept_file(_: &Path) -> bool {
        true
    }
    fn accept_dir(_: &Path) -> bool {
        false
    }
}

struct Sources;
impl Accept for Sources {
    fn accept_dir(p: &Path) -> bool {
        p.file_name().unwrap() == OsStr::new("src")
    }
}

fn main() {
    let mut all_iter = DirIter::<AllFiles>::new(&Path::new(".")).unwrap();
    while let Some(path) = all_iter.next() {
        println!(">> {:?}", path);
    }

    println!("{}", std::iter::repeat('-').take(80).collect::<String>());

    let iter = DirIter::<Sources>::new(&Path::new(".")).unwrap();
    for name in iter {
        println!(">> {:?}", name);
    }
}
