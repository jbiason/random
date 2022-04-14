use clap::Parser;

#[derive(Parser)]
struct Params {
    /// String opcional
    #[clap(long)]
    opt: Option<String>,

    /// String obrigatória
    #[clap(long)]
    obr: String,
}

fn main() {
    let args = Params::parse();

    println!("Obrigatório: {:?}", args.obr);
    println!("   Opcional: {:?}", args.opt);
}
