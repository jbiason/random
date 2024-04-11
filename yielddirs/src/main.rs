use std::fs::ReadDir;
use std::path::Path;
use std::path::PathBuf;

struct Walker {
    path: PathBuf
}

impl Walker {
    pub fn new(base: &Path) -> Self {
        println!("base={base:?}");
        println!("is_dir={}", base.is_dir());
        Self { path: base.to_path_buf()}
    }

    pub fn children(&self) -> WalkerIterator {
        WalkerIterator::new(&self.path)
    }
}

struct WalkerIterator {
    walking: ReadDir
}

impl WalkerIterator {
    fn new(path: &Path) -> Self {
        Self {
            walking: path.read_dir().unwrap()
        }
    }
}

impl Iterator for WalkerIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.walking.next()?;
        Some(current.unwrap().file_name().to_string_lossy().to_string())
    }
}

fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let base_path = args.next().unwrap();

    let walker = Walker::new(&Path::new(&base_path).canonicalize().unwrap());
    for child in walker.children() {
        println!("{child}");
    }
}
