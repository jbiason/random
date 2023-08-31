use std::ffi::OsString;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;

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

fn main() {
    // this requires always running with `cargo run`
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let log_file = File::create("script.log").unwrap();
    let the_script = base.join("src").join("the_script.sh");
    let bash = search_in_path("bash").unwrap();

    let mut cmd = Command::new(bash)
        .arg(the_script)
        .stdout(log_file)
        .spawn()
        .unwrap();

    cmd.wait().unwrap();
}
