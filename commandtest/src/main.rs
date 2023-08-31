use std::ffi::OsString;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
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
    let the_script = base.join("src").join("the_script.sh");
    let bash = search_in_path("bash").unwrap();

    let mut cmd = Command::new(bash)
        .arg(the_script)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = cmd.stdout.take().unwrap();
    let mut stderr = cmd.stderr.take().unwrap();

    let writer_pid = std::thread::spawn(move || {
        let reader = BufReader::new(stdout);
        let lines = reader.lines();
        let mut log_file = File::create("script.log").unwrap();
        let mut in_warning = false;
        let mut result = Vec::new();

        for line in lines {
            let line = line.unwrap();
            log_file.write(line.as_bytes()).unwrap();
            log_file.write(b"\n").unwrap(); // 'cause lines() eat it

            if line.starts_with("WARNING:") {
                in_warning = true;
            } else if line.starts_with("   ") && in_warning {
                result.push(line);
            } else if in_warning {
                in_warning = false;
            }
        }

        result
    });

    cmd.wait().unwrap();
    let warnings = writer_pid.join().unwrap();

    let mut buffer = String::new();
    stderr.read_to_string(&mut buffer).unwrap();

    let mut file = OpenOptions::new().append(true).open("script.log").unwrap();
    file.write(buffer.as_bytes()).unwrap();

    println!("Warnings:\n{:?}", warnings);
    println!("ERR:\n{:?}", buffer)
}
