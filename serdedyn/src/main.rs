use serde_derive::Deserialize;
use serde_derive::Serialize;

trait Marker {}

#[derive(Serialize, Deserialize, Debug)]
struct AsString {
    value: String,
}
impl Marker for AsString {}

#[derive(Serialize, Deserialize, Debug)]
struct AsU8 {
    value: u8,
}
impl Marker for AsU8 {}

fn main() {
    let mut contents: Vec<Box<dyn Marker>> = Vec::new();
    contents.push(Box::new(AsString {
        value: "Some value".into(),
    }));
    contents.push(Box::new(AsU8 { value: 2 }));

    let convered = toml::to_string(&contents).unwrap();
    println!("{}", convered);
}
