use std::{ops::Deref, path::{Path, PathBuf}};

fn not_zero(num: usize) -> Result<(), String> {
    if num == 0 {
        Err("It's zero!".into())
    } else {
        Ok(())
    }
}

trait BoolExt {
    fn ok(&self, fail: String) -> Result<(), String>;
}

impl BoolExt for bool {
    fn ok(&self, fail: String) -> Result<(), String> {
        if *self {
            Ok(())
        } else {
            Err(fail)
        }
    }
}

struct TestDir(PathBuf);
impl TestDir {
    pub fn new(name: &str) -> Self {
        let wd = std::env::temp_dir().join(name);
        std::fs::create_dir_all(&wd).unwrap();

        Self(wd)
    }
}
impl Deref for TestDir {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Drop for TestDir {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_result_ok() -> Result<(), ()> {
        Ok(())
    }

    #[test]
    fn with_result_err() -> Result<(), ()> {
        Err(())
    }

    #[test]
    fn nested() -> Result<(), String> {
        not_zero(1)?;
        Ok(())
    }

    #[test]
    fn nested2() -> Result<(), String> {
        not_zero(0)?;
        Ok(())
    }

    #[test]
    fn nested3() -> Result<(), String> {
        let result = (|| {
            not_zero(2)?;
            Ok(())
        })();

        // do something else, like cleaning up the testing workspace

        result
    }

    #[test]
    fn comparison() -> Result<(), String> {
        let result = (|| {
            not_zero(2)?;
            // Ok, this is weird:
            // bool.then_some() will convert true to Some(x) and false to None;
            // Option.ok_or() will convert the Option to Result
            // Result.map() will convert the Ok to some other value
            (1 == 1)
                .then_some(0)
                .ok_or("1 is not 1!".into())
                .map(|_| ())
        })();

        result
    }

    #[test]
    fn magic() -> Result<(), String> {
        let result = (|| {
            not_zero(2)?;
            (1 == 1).ok("1 is not 1".into())
        })();

        result
    }

    #[test]
    fn magic_dir() -> Result<(), String> {
        let wd = TestDir::new("magic-dir");
        let file1 = wd.join("file");
        std::fs::write(&file1, "this is file").map_err(|_| "No file!")?;
        assert!(1 == 1);
        assert!(!file1.is_file());
        Ok(())
    }
}
