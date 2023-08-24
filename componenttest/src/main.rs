use std::{
    ffi::OsStr,
    path::{Component, Path, PathBuf},
};

pub trait ExpandedExt {
    fn expand(&self) -> PathBuf;
}

impl ExpandedExt for Path {
    fn expand(&self) -> PathBuf {
        let mut buf = PathBuf::new();
        for component in self.components() {
            match component {
                Component::Normal(path) => match path.to_str() {
                    None => buf.push(path), // component can't be converted to str
                    Some(path_as_str) => {
                        match std::env::var_os(&path_as_str[1..]) {
                            None => buf.push(path),     // variable is not set
                            Some(value) => buf.push(value),
                        }
                    }
                },
                a => buf.push(a),
            }
        }
        buf
    }
}

fn main() {
    let path1 = PathBuf::from("/tmp/dir/file.txt");
    for comp in path1.components() {
        println!("{:?}", comp);
    }

    let path2 = PathBuf::from("$MAGIC_PATH/dir/file.txt");
    for comp in path2.components() {
        println!("{:?}", comp);
    }

    let new_path2 = path2
        .components()
        .map(|comp| match comp {
            Component::Normal(x) if x == "$MAGIC_PATH" => {
                Component::Normal(OsStr::new("magical_path_expanded"))
            }
            x => x,
        })
        .collect::<PathBuf>();
    println!("Path2: {:?}", new_path2);

    let path3 = PathBuf::from("$USER/Projects/Personal/random/$CARGO_MANIFEST_DIR");
    println!("Path3: {:?}", path3.expand());
}
