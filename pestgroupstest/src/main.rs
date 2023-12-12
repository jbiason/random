use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "csv.pest"]
struct CsvParser;

fn main() {
    let line = "(1 2 3) 123 123 (123 (123))";
    let mut parser = CsvParser::parse(Rule::csv, &line).unwrap();
    let _csv = parser.next().unwrap();
    let worker = _csv
        .into_inner()
        .filter_map(|pair| extract(pair))
        .collect::<Vec<_>>();
    println!("{:?}", worker);
}

fn extract(pair: pest::iterators::Pair<'_, Rule>) -> Option<Vec<&str>> {
    match pair.as_rule() {
        Rule::value => Some(vec![pair.as_str()]),
        Rule::group => {
            let local = pair.into_inner().filter_map(|pair| extract(pair)).flatten().collect::<Vec<_>>();
            // println!("{:?}", local);
            Some(local)
        }
        _ => None,
    }
}
