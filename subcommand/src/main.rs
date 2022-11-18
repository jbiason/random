use clap::Arg;
use clap::Command;

fn main() {
    let command = Command::new("example")
        .arg(
            Arg::new("working_directory")
                .short('w')
                .long("working-directory"),
        )
        .subcommand(
            Command::new("run")
                .about("Run tasks")
                .arg(Arg::new("cases")),
        );

    let matches = command.get_matches();

    match matches.subcommand() {
        Some(("run", _sub_matches)) => println!("It's run"),
        _ => {
            println!("Finding out if it an external command...")
        }
    }
}
