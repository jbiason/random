use std::env;
use std::path::PathBuf;

fn split(line: &str) -> Option<(String, Vec<String>)> {
    let mut split = line.split(' ');
    let root = split.next()?;
    let arguments = split.map(|x| x.to_string()).collect();
    Some((root.to_string(), arguments))
}

fn as_command(command: &str) -> Option<PathBuf> {
    env::split_paths(&env::var_os("PATH")?).find_map(|dir| {
        let full_path = dir.join(command);
        if full_path.exists() {
            Some(full_path)
        } else {
            None
        }
    })
}

fn main() {
    let commands = ["ls -la", "ls", "exa", "not"];
    let result = commands
        .iter()
        .map(|line| split(&line))
        .map(|opt| {
            let (command, args) = opt?;
            let full_command = as_command(&command)?;
            Some((full_command, args))
        })
        .map(|opt| opt.ok_or("Not good enough"))
        .collect::<Result<Vec<_>, _>>();
    println!("Result: {:?}", result)
}
