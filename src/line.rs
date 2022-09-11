use std::fmt;

use crate::utils::remove_whitespace;

#[derive(PartialEq, Eq)]
pub enum InstructionType {
    C,
    A,
    L,
}

pub struct SourceLine {
    pub line_num: usize,
    pub value: String,
}

impl SourceLine {
    pub fn new(line_num: usize, source: String) -> Self {
        let value = remove_whitespace(&source);
        Self { line_num, value }
    }
}

impl fmt::Display for SourceLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.line_num, self.value)
    }
}

pub struct Instruction {
    line_num: usize,
    pub inst_type: InstructionType,
    pub binary_str: String,
}

impl Instruction {
    pub fn new(source: &SourceLine) -> Self {
        let inst_type = if source.value.starts_with('@') {
            InstructionType::A
        } else if source.value.starts_with('(') {
            InstructionType::L
        } else {
            InstructionType::C
        };

        Self {
            line_num: source.line_num,
            inst_type,
            binary_str: String::new(),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.line_num, &self.binary_str.clone())
    }
}
