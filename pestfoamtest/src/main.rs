use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "foam.pest"]
pub struct Foam;

fn main() {
    let parse = Foam::parse(Rule::multi_comment, "/* this is comment */");
    println!("{:?}", parse);
    let parse = Foam::parse(Rule::single_comment, "// this is comment");
    println!("{:?}", parse);
    let parse = Foam::parse(Rule::field, "-273.15");
    println!("{:?}", parse);
}
