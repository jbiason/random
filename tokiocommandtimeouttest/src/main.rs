use std::ffi::OsString;
use std::path::PathBuf;
use std::time::Duration;

use tokio::process::Command;
use tokio::time::timeout;

/// Special script that looks for some file in $PATH.
fn search_in_path(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH").unwrap_or_else(OsString::new);
    std::env::split_paths(&path).find_map(|dir| {
        let full_path = dir.join(name);
        if full_path.is_file() {
            Some(full_path)
        } else {
            None
        }
    })
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let sleep = search_in_path("sleep").unwrap();

    println!("Run 3 secs");
    let mut cmd = Command::new(&sleep).arg("3s").spawn().unwrap();
    if let Err(_) = timeout(Duration::from_secs(4), cmd.wait()).await {
        println!("Got timeout!");
        cmd.kill().await.unwrap();
    } else {
        println!("No timeout");
    }

    println!("Run 25 secs");
    let mut cmd = Command::new(&sleep).arg("25s").spawn().unwrap();
    if let Err(_) = timeout(Duration::from_secs(4), cmd.wait()).await {
        println!("Got timeout");
        cmd.kill().await.unwrap();
    } else {
        println!("No timeout");
    }
}
