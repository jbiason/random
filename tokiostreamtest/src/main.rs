use std::path::{Path, PathBuf};

use async_stream::stream;
use futures_core::Stream;
use futures_util::{StreamExt, pin_mut};

fn filenames(start: &Path) -> impl Stream<Item = PathBuf> + '_ {
    stream! {
        let contents = tokio::fs::read_dir(start).await;
        if let Ok(mut reader) = contents {
            while let Ok(Some(entry)) = reader.next_entry().await {
                let entry = entry.path();
                if entry.is_file() {
                    yield entry;
                }
            }
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let name_stream = filenames(Path::new("src"));
    pin_mut!(name_stream);

    while let Some(name) = name_stream.next().await {
        println!("Found {:?}", name);
    }
    println!("Hello, world!");
}
