use std::sync::RwLock;

struct Holder {
    content: RwLock<String>,
}

impl Holder {
    fn new() -> Self {
        Self {
            content: RwLock::new(String::from("hello")),
        }
    }

    fn change(&self, content: &str) {
        let mut original = self.content.write().unwrap();
        *original = content.into();
    }

    fn content(&self) -> String {
        let lock = self.content.read().unwrap();
        String::from(&*lock)
    }
}

fn main() {
    let content = Holder::new();
    println!("Content: {}", content.content());

    content.change("Hello there");
    println!("Content: {}", content.content());
}
