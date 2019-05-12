pub mod config;

use std::path::{Path};
use std::io::{Stdin};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct FileInput {
    delim: char,
    reader: BufReader<File>,
}

impl FileInput {
    pub fn new<T: AsRef<Path>>(path: T, delim: char) -> io::Result<FileInput> {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        Ok(
            FileInput {
                delim,
                reader,
            }
        )
    }

    pub fn by_newline<T: AsRef<Path>>(path: T) -> io::Result<FileInput> {
        Self::new(path, '\n')
    }
}

impl InputStream for FileInput {
    fn next_by_delim(&mut self) -> Option<String> {
        let mut buf = Vec::new();
        let len = self.reader.read_until(self.delim as u8, &mut buf);
        if len.is_ok() && len.unwrap() > 0 {
            String::from_utf8(buf).ok()
        } else {
            None
        }
    }
}

pub struct StdinInput {
    delim: char,
    stdin: Stdin,
}

impl StdinInput {
    pub fn new(delim: char) -> Self {
        let stdin = io::stdin();
        StdinInput {
            delim,
            stdin,
        }
    }

    pub fn by_newline() -> Self {
        Self::new('\n')
    }
}

impl InputStream for StdinInput {
    fn next_by_delim(&mut self) -> Option<String> {
        let mut buf = Vec::new();
        let mut handle = self.stdin.lock();
        let len = handle.read_until(self.delim as u8, &mut buf);
        if len.is_ok() && len.unwrap() > 0 {
            String::from_utf8(buf).ok()
        } else {
            None
        }
    }
}

pub struct Stream<I: InputStream> {
    _priv: I
}

impl<I: InputStream> From<I> for Stream<I> {
    fn from(source: I) -> Stream<I> {
        Stream {
            _priv: source,
        }
    }
}

pub trait InputStream {
    fn next_by_delim(&mut self) -> Option<String>;
}

impl<I: InputStream> Iterator for Stream<I> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        self._priv.next_by_delim()
    }
}
