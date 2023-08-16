use std::path::PathBuf;

use transformer::Generator;
// use transformer::Processor;
use transformer::Source;
// use transformer::Transformer;

mod transformer;

struct Input {}
#[async_trait::async_trait]
impl Generator for Input {
    type Output = PathBuf;

    async fn next(&self) -> Option<PathBuf> {}
}

// struct PathBufToString {}
// impl Processor for PathBufToString {
//     type Input = PathBuf;
//     type Output = String;

//     fn transform(&self, data: PathBuf) -> String {
//         data.to_string_lossy().to_string()
//     }
// }

// struct Printer {}
// impl Processor for Printer {
//     type Input = String;
//     type Output = ();   // this is a sync

//     fn transform(&self, data: String) -> () {
//         println!("{:?}", data);
//     }
// }

fn main() {
    let gen = Source::new(Input {});
    // let pbts = Transformer::new(PathBufToString {});
    // let printer = Transformer::new(Printer {});

    // gen.chain(pbts);
    // pbts.chain(printer);

    // run(printer, pbts, gen);
}
