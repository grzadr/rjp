// src/input_source.rs

use std::io::{self, BufRead, BufReader};
use std::fs::File;

pub enum InputSource {
    Stdin(BufReader<io::Stdin>),
    File(BufReader<File>),
}

impl InputSource {
    pub fn new_stdin() -> Self {
        InputSource::Stdin(BufReader::new(io::stdin()))
    }

    pub fn new_file(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        Ok(InputSource::File(BufReader::new(file)))
    }

    pub fn lines(&mut self) -> io::Result<String> {
        let mut line = String::new();
        match self {
            InputSource::Stdin(stdin) => stdin.read_line(&mut line)?,
            InputSource::File(file) => file.read_line(&mut line)?,
        };
        Ok(line)
    }
}
