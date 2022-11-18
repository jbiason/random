// This doesnt works:

use clap::Arg;
use clap::Command;

fn main() {
    let command = Command::new("example")
        .subcommand_required(true)
        .allow_external_subcommands(true)
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
        Some((ext, _sub_matches)) => println!("Finding if we have a \"{ext}\" command"),
        _ => {
            println!("Finding out if it an external command...")
        }
    }
}

// This doesn't work

// use std::path::{Path, PathBuf};

// use clap::{Parser, Subcommand};

// #[derive(Parser)]
// #[command(version)]
// struct Cli {
//     #[arg(short, long)]
//     working_directory: Option<PathBuf>,

//     #[command(subcommand)]
//     command: Command,
// }

// #[derive(Subcommand)]
// enum Command {
//     Run { cases: Option<Vec<String>> },
// }

// fn main() {
//     let cli = Cli::parse();
//     match &cli.command {
//         Command::Run { cases } => println!("Running cases: {:?}", cases),
//         _ => println!("External"),
//     }
// }
