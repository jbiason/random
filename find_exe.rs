use std::env;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let args = env::args();
    if args.len() != 2 {
        println!("Missing executable name");
        return;
    }

    let command = PathBuf::from(&args.last().unwrap());
    println!("Searching for {:?}...", command);

    if command.exists() {
        println!("Command is here!");
        return;
    }

    println!("Exe is in {:?}", find_in_path(&command));
}

fn find_in_path(command: &Path) -> Option<PathBuf> {
    env::split_paths(&env::var_os("PATH")?)
        .filter_map(|dir| {
            let full_path = dir.join(&command);
            if full_path.exists() {
                Some(full_path)
            } else {
                None
            }
        })
        .next()
}
