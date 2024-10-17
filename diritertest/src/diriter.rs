//! Iterator over directories

use std::collections::VecDeque;
use std::fs::ReadDir;
use std::marker::PhantomData;
use std::path::Path;
use std::path::PathBuf;

pub trait Accept {
    /// Accept a directory and return it as a result. Note that this stops the processing of any
    /// other entries in the directory.
    fn accept_dir(_: &Path) -> bool {
        false
    }

    /// Accept a file and return it as a result.
    fn accept_file(_: &Path) -> bool {
        false
    }
}

pub struct DirIter<A: Accept> {
    curr_dir: ReadDir,
    remaining_dirs: VecDeque<PathBuf>,
    accept: PhantomData<A>,
}

impl<A> DirIter<A>
where
    A: Accept,
{
    pub fn new(start: &Path) -> std::io::Result<Self> {
        Ok(Self {
            curr_dir: start.read_dir()?,
            remaining_dirs: VecDeque::new(),
            accept: PhantomData,
        })
    }

    pub fn advance(&mut self) -> Option<PathBuf> {
        let next = self.curr_dir.next();
        match next {
            Some(Ok(x)) => {
                let x = x.path();
                if x.is_dir() {
                    if A::accept_dir(&x) {
                        Some(x)
                    } else {
                        self.remaining_dirs.push_back(x);
                        self.advance()
                    }
                } else {
                    if A::accept_file(&x) {
                        Some(x)
                    } else {
                        self.advance()
                    }
                }
            }
            _ => {
                let next_dir = self.remaining_dirs.pop_front()?;
                self.curr_dir = next_dir.read_dir().ok()?;
                self.advance()
            }
        }
    }
}

impl<A: Accept> Iterator for DirIter<A> {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance()
    }
}
