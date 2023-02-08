#[derive(Debug)]
enum MyType {
    SomethingWrong,
    SomethingGood,
}

enum SomeError {
    Bad,
}

impl From<SomeError> for MyType {
    fn from(value: SomeError) -> Self {
        MyType::SomethingWrong
    }
}

fn validate(s: &str) -> Result<(), SomeError> {
    if s == "bad" {
        Err(SomeError::Bad)
    } else {
        Ok(())
    }
}

fn check(response: &str) -> MyType {
    let inner = validate(&response)?;
    println!("Inner: {:?}", inner);
    inner
}

fn main() {
    let results = ["good", "bad", "good"];
    for variant in results {
        let response = check(&variant);
        println!("{} = {:?}", variant, response);
    }
}
