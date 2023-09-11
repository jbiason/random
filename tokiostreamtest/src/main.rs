use std::path::{Path, PathBuf};

use async_stream::stream;
use futures_core::Stream;
use futures_util::{StreamExt, pin_mut};

#[async_recursion::async_recursion]
async fn filenames(start: &Path) -> impl Stream<Item = PathBuf> + '_ {
    stream! {
        let contents = tokio::fs::read_dir(start).await;
        if let Ok(mut reader) = contents {
            while let Ok(Some(entry)) = reader.next_entry().await {
                let entry = entry.path();
                if entry.is_file() {
                    yield entry;
                } else if entry.is_dir() {
                    // And this doesn't work 'cause we can't yield its results and we can't return
                    // it due the return types being different.
                    let under_dir = filenames(&entry).await;
                    pin_mut!(under_dir);
                    while let Some(name) = under_dir.next().await {
                        yield name
                    }
                }
            }
        }
    }
}

fn sync_filenames(start: &Path) -> impl Stream<Item = PathBuf> + '_ {
    stream! {
        let contents = start.read_dir();
        if let Ok(reader) = contents {
            for entry in reader {
                let entry = entry.unwrap().path();
                if entry.is_file() {
                    yield entry;
                } else if entry.is_dir() {
                    let under_dir = sync_filenames(&entry);
                    pin_mut!(under_dir);

                    while let Some(name) = under_dir.next().await {
                        yield name
                    }
                }
            }
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // let name_stream = filenames(Path::new(".")).await;
    // pin_mut!(name_stream);
    let name_stream = sync_filenames(Path::new("."));
    pin_mut!(name_stream);

    while let Some(name) = name_stream.next().await {
        println!("Found {:?}", name);
    }
    println!("Hello, world!");
}
