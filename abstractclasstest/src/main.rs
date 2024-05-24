trait Base {
    fn files(&self) -> Vec<String>;
    fn show_files(&self) {
        for file in self.files() {
            println!("file={file}");
        }
    }
}

struct UpcaseFiles {}
impl Base for UpcaseFiles {
    fn files(&self) -> Vec<String> {
        vec!["HELLO".into(), "WORLD".into()]
    }
}

struct LowcaseFiles {}
impl Base for LowcaseFiles {
    fn files(&self) -> Vec<String> {
        vec!["hello".into(), "world".into()]
    }
}

fn main() {
    let f1 = UpcaseFiles {};
    f1.show_files();

    let f2 = LowcaseFiles {};
    f2.show_files()
}
