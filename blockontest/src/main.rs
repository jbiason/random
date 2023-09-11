use std::{
    future::Future,
    path::{Path, PathBuf},
};

use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader}, runtime::Handle,
};

pub async fn async_walk_files<F, T>(start: &Path, on_file: T)
where
    T: Fn(PathBuf) -> F + Send + std::marker::Sync + std::marker::Copy,
    F: Future<Output = ()> + Send,
{
    inner_async_walk_files(start, start, on_file).await;
}

#[async_recursion::async_recursion]
async fn inner_async_walk_files<T, F>(base: &Path, current: &Path, on_file: T)
where
    T: Fn(PathBuf) -> F + Send + std::marker::Sync + std::marker::Copy,
    F: Future<Output = ()> + Send,
{
    println!("Walk: {:?}", current);
    let dir_contents = tokio::fs::read_dir(current).await;
    if let Ok(mut content) = dir_contents {
        while let Ok(Some(entry)) = content.next_entry().await {
            let entry = entry.path();
            if entry.is_file() {
                on_file(entry).await;
            } else if entry.is_dir() {
                inner_async_walk_files(base, &entry, on_file).await;
            }
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    async_walk_files(Path::new(".."), |path| async move {
        // This doesn't work 'cause, on a single thread runtime, the working thread would be
        // blocked, and Tokio doesn't like it.
        let handle = Handle::current();
        handle.block_on(async move {
            println!("File: {:?}", path);
            let file = File::open(&path).await.unwrap();
            let reader = BufReader::new(file);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                println!("Line: {path:?} -> {line:?}");
            }
        })
    })
    .await;
}
