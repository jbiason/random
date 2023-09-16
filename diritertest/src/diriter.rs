//! Iterator over directories

use std::collections::VecDeque;
use std::fs::DirEntry;
use std::fs::ReadDir;
use std::path::Path;
use std::path::PathBuf;

pub struct DirIter {
    curr_dir: ReadDir,
    remaining_dirs: VecDeque<PathBuf>,
}

impl DirIter {
    pub fn new(start: &Path) -> std::io::Result<Self> {
        Ok(Self {
            curr_dir: start.read_dir()?,
            remaining_dirs: VecDeque::new(),
        })
    }

    pub fn advance(&mut self) -> Option<PathBuf> {
        let next = self.curr_dir.next();
        match next {
            Some(Ok(x)) => {
                if x.path().is_dir() {
                    self.remaining_dirs.push_back(x.path());
                    self.advance()
                } else {
                    Some(x.path())
                }
            }
            Some(Err(_)) | None => {
                let next_dir = self.remaining_dirs.pop_front()?;
                self.curr_dir = std::fs::read_dir(next_dir).ok()?;
                self.advance()
            }
        }
    }
}

impl Iterator for DirIter {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance()
    }
}
