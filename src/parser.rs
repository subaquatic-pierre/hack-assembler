use std::fs::File;
use std::io::{self, BufRead};

use crate::line::SourceLine;
use crate::symbols::Symbols;
use crate::utils::rem_first_and_last;

pub struct Parser {
    filename: String,
    pub source: Vec<SourceLine>,
}

impl Parser {
    pub fn new(filename: String) -> Self {
        Self {
            filename,
            source: Vec::new(),
        }
    }

    pub fn read_lines(&mut self) -> Result<Symbols, io::Error> {
        let mut symbols = Symbols::new();
        if let Ok(lines) = self._read_lines() {
            // create line struct
            let mut line_num: usize = 0;
            for line in lines.flatten() {
                // check if line has comment
                let cmt_split: Vec<&str> = line.split("//").collect();
                if line.starts_with('/') || line.is_empty() {
                    continue;
                } else if line.starts_with('(') {
                    let label = rem_first_and_last(line.to_string());
                    let line = SourceLine::new(line_num, cmt_split.get(0).unwrap().to_string());
                    symbols.label.insert(label, line_num as i32);
                    self.source.push(line);
                } else {
                    let line = SourceLine::new(line_num, cmt_split.get(0).unwrap().to_string());
                    self.source.push(line);
                    line_num += 1;
                }
            }
        }

        Ok(symbols)
    }

    fn _read_lines(&self) -> io::Result<io::Lines<io::BufReader<File>>> {
        let file = File::open(format!("./{}", self.filename))?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn print_source(&self) {
        println!("\n --- SOURCE CODE --- \n");
        for line in &self.source {
            println!("{line}");
        }
        println!("\n --- END --- \n");
    }
}
