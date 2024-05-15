use std::ffi::OsStr;
use std::path::Path;

fn extract<'a>(
    path: &'a Path,
    current_name: Option<&'a str>,
) -> Option<(&'a Path, Option<&'a str>)> {
    let name = path.file_name().map(OsStr::to_str).flatten();
    let parent = path.parent()?;

    // println!("path={path:?}, current_name={current_name:?}, parent={parent:?}, name={name:?}");

    if name == Some(".run") {
        Some((parent, current_name.or(Some("default"))))
    } else {
        extract(&parent, current_name.or(name))
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base() {
        let result = extract(Path::new("something/.run"), None).unwrap();
        assert_eq!(result.0, Path::new("something"));
        assert_eq!(result.1, Some("default"));
    }

    #[test]
    fn child() {
        let result = extract(Path::new("something/.run/special/path"), None).unwrap();
        assert_eq!(result.0, Path::new("something"));
        assert_eq!(result.1, Some("path"));
    }

    #[test]
    fn broken() {
        let result = extract(Path::new("something/fun/for/everyone"), None);
        assert!(result.is_none());
    }
}
