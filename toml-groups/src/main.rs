use serde_derive::Serialize;
use serde_derive::Deserialize;

#[derive(Serialize, Deserialize)]
struct Child {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Main {
    context: String,
    child: Child,
}

fn main() {
    let example = Main { context: "Main".to_string(),
                         child: Child { name: "child".to_string() } };
    let content = toml::to_string(&example).unwrap();

    println!("{:?}", content);
}
