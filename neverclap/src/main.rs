use clap::Parser;

#[derive(clap::Subcommand, Debug)]
enum Actions {
    Generate,
}

#[derive(Parser, Debug)]
struct Config {
    #[command(subcommand)]
    generate: Option<Actions>,

    #[arg(short, long, env = "ENV_VAR")]
    var: Option<String>,
}

fn main() {
    let args = Config::parse();
    println!("Args: {:?}", args)
}
