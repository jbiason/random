use std::ffi::OsString;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::PathBuf;
use std::process::Command;

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

    // capture both the stdout and stderr as File structs (actually FDs, but basically the same
    // thing)
    let stdout = cmd.stdout.take().unwrap();
    let mut stderr = cmd.stderr.take().unwrap();

    // spawn a thread to keep capturing and processing the stdout.
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

    // run the command till it finishes
    cmd.wait().unwrap();

    // ... and wait till the thread finishes processing the whole output.
    let warnings = writer_pid.join().unwrap();

    // this is somewhat a hack: Instead of spawning a thread for stderr and trying to fight with
    // stdout for the lock to be able to write in the log file, we do this after the thread ends
    // (which closes the file) and then open it again and write the stderr in the end. We do this
    // 'cause we expect that the stderr is way smaller than stdout and can fit in memory without
    // any issues.
    let mut buffer = String::new();
    stderr.read_to_string(&mut buffer).unwrap();

    let mut file = OpenOptions::new().append(true).open("script.log").unwrap();
    file.write(buffer.as_bytes()).unwrap();

    // This is purely for diagnostic purposes. We could put the warnings in another file, or pass
    // it along to something else to process it. Here, we just display them.
    // Same for stderr: Since we already put them in the file, this is used just to make sure we
    // are capturing the errors without looking at the file.
    println!("Warnings:\n{:?}", warnings);
    println!("ERR:\n{:?}", buffer)
}
