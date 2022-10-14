struct Test {
    value1: String,
    value2: Vec<String>,
}

fn main() {
    let a = Test {
        value1: "string".into(),
        value2: vec!["a".into(), "b".into()],
    };

    let Test {
        value1,
        value2: list,
    } = a;

    println!("Name: {:?}", value1);
    println!("List: {:?}", list);
}
