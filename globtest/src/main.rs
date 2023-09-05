use std::path::{Path, PathBuf};

use regex::Regex;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut args = std::env::args();
    args.next(); // jump the command name.

    let source = PathBuf::from(args.next().expect("I want a place to copy from!"));
    let target = PathBuf::from(args.next().expect("I want a place to copy to!"));

    println!("From {source:?} to {target:?}");

    match (source.is_file(), source.is_dir()) {
        (true, true) => {
            println!("NO, WAIT! HOW THE THING IS A FILE AND DIR AT THE SAME TIME?!?!");
        }
        (true, false) => {
            println!("Source is file");
            copy_file(&source, &target).await;
        }
        (false, true) => {
            println!("Source is a whole directory");
            copy_dir(&source, &target).await;
        }
        (false, false) => {
            println!("Source is not a file or a directory");
            copy_magic(&source, &target).await
        }
    }
}

/// Magical copy
async fn copy_magic(source: &Path, target: &Path) {
    if let Some(name) = source.file_name() {
        if name.to_str().unwrap().contains('*') {
            copy_mask(source, target).await;
        } else {
            println!("Source is not a file, not a dir and it doesn't have a mask.");
            println!("I think you're nuts");
        }
    }
}

async fn copy_mask(source: &Path, target: &Path) {
    let mask = source.file_name().unwrap();
    let source = source.parent().unwrap();
    let re = Regex::new(&mask.to_str().unwrap().replace("*", ".*")).unwrap();

    let mut reader = tokio::fs::read_dir(&source).await.unwrap();
    while let Ok(Some(entry)) = reader.next_entry().await {
        let entry = entry.path();
        if entry.is_file() {
            if let Some(name) = entry.file_name() {
                // does the name match the mask?
                if re.is_match(name.to_str().unwrap()) {
                    copy_file(&source.join(name), target).await;
                }
            }
        }
    }
}

/// Make a copy of a directory
#[async_recursion::async_recursion]
async fn copy_dir(source: &Path, target: &Path) {
    if !target.is_dir() {
        println!("Can't copy a whole directory to something that it is NOT a directory!");
        return;
    }

    let mut reader = tokio::fs::read_dir(&source).await.unwrap();
    while let Ok(Some(entry)) = reader.next_entry().await {
        let entry = entry.path();
        if entry.is_file() {
            copy_file(&entry, &target).await;
        } else {
            let name = entry.file_name().unwrap();
            let target = target.join(name);
            tokio::fs::create_dir_all(&target).await.unwrap();
            copy_dir(&entry, &target).await;
        }
    }
}

/// Make a copy of a file.
async fn copy_file(source: &Path, target: &Path) {
    if target.is_dir() {
        println!("Copying to a directory");
        let filename = source.file_name().unwrap(); // after all, it IS a file
        let target = target.join(filename);
        println!("Copying from {:?} to {:?}", source, target);
        tokio::fs::copy(source, target).await.unwrap();
    } else {
        println!("Assuming the destination is already the filename");
        println!("Copying from {:?} to {:?}", source, target);
        tokio::fs::copy(source, target).await.unwrap();
    }
}
