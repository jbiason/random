use std::path::Path;
use std::path::PathBuf;

use tokio::fs::ReadDir;

struct Walker {
    path: PathBuf,
}

impl Walker {
    pub fn new(base: &Path) -> Self {
        Self {
            path: base.to_path_buf(),
        }
    }

    pub async fn children(&self) -> WalkerIterator {
        WalkerIterator::new(&self.path).await
    }
}

struct WalkerIterator {
    walking: ReadDir,
}

impl WalkerIterator {
    async fn new(path: &Path) -> Self {
        Self {
            walking: tokio::fs::read_dir(&path).await.unwrap(),
        }
    }

    async fn next(&mut self) -> Option<String> {
        let current = self.walking.next_entry().await.ok()??;
        Some(current.file_name().to_string_lossy().to_string())
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let base_path = args.next().unwrap();

    let walker = Walker::new(&Path::new(&base_path).canonicalize().unwrap());
    let mut iter = walker.children().await;
    while let Some(child) = iter.next().await {
        println!("{child}");
    }

    //while let Some(child) = walker.children().await.next().await {
    //    println!("{child}")
    //}
}
