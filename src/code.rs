use std::convert::TryInto;

use crate::line::{Instruction, InstructionType, SourceLine};
use crate::symbols::Symbols;
use crate::utils::rem_first_and_last;

pub struct Code<'a> {
    symbols: &'a mut Symbols,
}

#[derive(Debug)]
pub struct InstructionToken {
    pub dest: String,
    pub comp: String,
    pub jump: String,
}

impl<'a> Code<'a> {
    pub fn new(symbols: &'a mut Symbols) -> Self {
        Self { symbols }
    }

    pub fn instruction_from_source(&mut self, source: SourceLine) -> Instruction {
        let mut instruction = Instruction::new(&source);

        match instruction.inst_type {
            InstructionType::A => {
                let inst_str = source.value[1..].to_string();

                // check if inst in symbols builtin
                let addr = match self.symbols.built_in.get(&inst_str) {
                    Some(&addr) => addr,
                    None => {
                        // check if inst addr is number
                        let addr = match &inst_str.parse::<i32>() {
                            Ok(addr) => *addr,
                            Err(_) => {
                                // check if addr is label
                                let addr = match self.symbols.label.get(&inst_str) {
                                    Some(label_addr) => *label_addr,
                                    None => {
                                        // find var in symbols
                                        // check if var exists in symbols
                                        // get address of var from symbols
                                        let var_addr = match self.symbols.variable.get(&inst_str) {
                                            Some(&addr) => addr,
                                            None => {
                                                // if var doesnt exist add to symbol table
                                                // get decimal value of address
                                                let var_index =
                                                    (self.symbols.variable.len() + 16) as i32;
                                                self.symbols.variable.insert(inst_str, var_index);
                                                var_index
                                            }
                                        };
                                        var_addr
                                    }
                                };
                                addr
                            }
                        };
                        addr
                    }
                };

                // convert decimal to string
                let bin_str = format!("{:0>16}", format!("{:b}", addr));
                instruction.binary_str = bin_str;
            }
            InstructionType::L => {
                // parse label value
                let label = rem_first_and_last(source.value[1..].to_string());

                // check if label exists in symbols
                let addr = match self.symbols.label.get(&label) {
                    Some(label_addr) => *label_addr,
                    None => {
                        self.symbols
                            .label
                            .insert(label.to_string(), source.line_num.try_into().unwrap());
                        source.line_num as i32
                    }
                };

                let str_addr = format!("{:0>16}", format!("{:b}", addr));
                instruction.binary_str = str_addr;
            }
            InstructionType::C => {
                let tokens = self.parse_c_ins_tokens(&source);
                let mut binary_string = "111".to_string();

                // comp
                if let Some(comp_bin) = self.symbols.comp.get(&tokens.comp) {
                    binary_string.push_str(comp_bin);
                }

                // dest
                if let Some(dest_bin) = self.symbols.dest.get(&tokens.dest) {
                    binary_string.push_str(dest_bin);
                } else {
                    binary_string.push_str("000")
                }

                // jump
                if let Some(jump_bin) = self.symbols.jump.get(&tokens.jump) {
                    binary_string.push_str(jump_bin);
                } else {
                    binary_string.push_str("000")
                }

                // println!("{:?}", tokens);

                instruction.binary_str = binary_string;
            }
        }

        instruction
    }

    fn parse_c_ins_tokens(&self, source: &SourceLine) -> InstructionToken {
        // split string on =
        let org_str = source.value.clone();

        let mut jump = String::new();
        let mut comp = String::new();
        let mut dest = String::new();

        // get dest val
        let jmp_split: Vec<&str> = org_str.split(';').collect();

        if jmp_split.len() == 1 {
            jump = "null".to_string();
        } else {
            // Check if jump exists
            if let Some(jump_tmp) = jmp_split.get(1) {
                jump = jump_tmp.to_string();
            }
        }

        let start_ins = jmp_split.get(0).unwrap();
        let rest_split: Vec<&str> = start_ins.split('=').collect();

        // check if dest exists
        if rest_split.len() == 1 {
            // no dest value
            dest = "null".to_string();
            comp = rest_split.get(0).unwrap().to_string();
        } else {
            // both dest and comp exist
            // get comp
            if let Some(dest_tmp) = rest_split.get(0) {
                dest = dest_tmp.to_string();
            }
            // get jump
            if let Some(comp_tmp) = rest_split.get(1) {
                comp = comp_tmp.to_string();
            }
        }

        InstructionToken { dest, jump, comp }
    }
}
