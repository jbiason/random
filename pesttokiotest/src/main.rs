use std::path::Path;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "csv.pest"]
struct CsvParser;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut args = std::env::args();
    let _ = args.next().unwrap();
    let filename = args.next().unwrap();

    tokio::spawn(async move {
        let content = get(&Path::new(&filename)).await;
        println!("{:?}", content);
        let parsed = parse(&content);
        println!("{:?}", parsed);
    })
    .await
    .unwrap();
}

async fn get(filename: &Path) -> String {
    let bytes = tokio::fs::read(filename).await.unwrap();
    String::from_utf8_lossy(&bytes).trim().to_string()
}

fn parse(content: &str) -> Vec<Vec<&str>> {
    let mut parser = CsvParser::parse(Rule::csv, content).unwrap();
    let root = parser.next().unwrap();
    root.into_inner()
        .filter_map(|pair| extract(pair))
        .collect::<Vec<_>>()
}

fn extract(pair: pest::iterators::Pair<'_, Rule>) -> Option<Vec<&str>> {
    match pair.as_rule() {
        Rule::value => Some(vec![pair.as_str()]),
        Rule::group => {
            let local = pair
                .into_inner()
                .filter_map(|pair| extract(pair))
                .flatten()
                .collect::<Vec<_>>();
            Some(local)
        }
        _ => None,
    }
}
