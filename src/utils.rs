use std::collections::HashMap;

pub fn build_built_in() -> HashMap<String, i32> {
    let mut symbols = HashMap::new();

    symbols.insert("R0".to_string(), 0);
    symbols.insert("R1".to_string(), 1);
    symbols.insert("R2".to_string(), 2);
    symbols.insert("R3".to_string(), 3);
    symbols.insert("R4".to_string(), 4);
    symbols.insert("R5".to_string(), 5);
    symbols.insert("R6".to_string(), 6);
    symbols.insert("R7".to_string(), 7);
    symbols.insert("R8".to_string(), 8);
    symbols.insert("R9".to_string(), 9);
    symbols.insert("R10".to_string(), 10);
    symbols.insert("R11".to_string(), 11);
    symbols.insert("R12".to_string(), 12);
    symbols.insert("R13".to_string(), 13);
    symbols.insert("R14".to_string(), 14);
    symbols.insert("R15".to_string(), 15);

    symbols.insert("SCREEN".to_string(), 16384);
    symbols.insert("KBD".to_string(), 24576);
    symbols.insert("SP".to_string(), 0);
    symbols.insert("LCL".to_string(), 1);
    symbols.insert("ARG".to_string(), 2);
    symbols.insert("THIS".to_string(), 3);
    symbols.insert("THAT".to_string(), 4);

    symbols
}

pub fn build_comp() -> HashMap<String, String> {
    let mut symbols = HashMap::new();

    symbols.insert("0".to_string(), "0101010".to_string());
    symbols.insert("1".to_string(), "0111111".to_string());
    symbols.insert("-1".to_string(), "0111010".to_string());
    symbols.insert("D".to_string(), "0001100".to_string());
    symbols.insert("A".to_string(), "0110000".to_string());
    symbols.insert("M".to_string(), "1110000".to_string());
    symbols.insert("!D".to_string(), "0001101".to_string());
    symbols.insert("!A".to_string(), "0110001".to_string());
    symbols.insert("!M".to_string(), "1110001".to_string());
    symbols.insert("-D".to_string(), "0001111".to_string());
    symbols.insert("-A".to_string(), "0110011".to_string());
    symbols.insert("-M".to_string(), "1110011".to_string());
    symbols.insert("D+1".to_string(), "0011111".to_string());
    symbols.insert("A+1".to_string(), "0110111".to_string());
    symbols.insert("M+1".to_string(), "1110111".to_string());
    symbols.insert("D-1".to_string(), "0001110".to_string());
    symbols.insert("A-1".to_string(), "0110010".to_string());
    symbols.insert("M-1".to_string(), "1110010".to_string());
    symbols.insert("D+A".to_string(), "0000010".to_string());
    symbols.insert("D+M".to_string(), "1000010".to_string());
    symbols.insert("D-A".to_string(), "0010011".to_string());
    symbols.insert("D-M".to_string(), "1010011".to_string());
    symbols.insert("A-D".to_string(), "0000111".to_string());
    symbols.insert("M-D".to_string(), "1000111".to_string());
    symbols.insert("D&A".to_string(), "0000000".to_string());
    symbols.insert("D&M".to_string(), "1000000".to_string());
    symbols.insert("D&M".to_string(), "1000000".to_string());
    symbols.insert("D|A".to_string(), "0010101".to_string());
    symbols.insert("D|M".to_string(), "1010101".to_string());

    symbols
}

pub fn build_jump() -> HashMap<String, String> {
    let mut symbols = HashMap::new();

    symbols.insert("null".to_string(), "000".to_string());
    symbols.insert("JGT".to_string(), "001".to_string());
    symbols.insert("JEQ".to_string(), "010".to_string());
    symbols.insert("JGE".to_string(), "011".to_string());
    symbols.insert("JLT".to_string(), "100".to_string());
    symbols.insert("JNE".to_string(), "101".to_string());
    symbols.insert("JLE".to_string(), "110".to_string());
    symbols.insert("JMP".to_string(), "111".to_string());

    symbols
}

pub fn build_dest() -> HashMap<String, String> {
    let mut symbols = HashMap::new();

    symbols.insert("null".to_string(), "000".to_string());
    symbols.insert("M".to_string(), "001".to_string());
    symbols.insert("D".to_string(), "010".to_string());
    symbols.insert("MD".to_string(), "011".to_string());
    symbols.insert("A".to_string(), "100".to_string());
    symbols.insert("AM".to_string(), "101".to_string());
    symbols.insert("AD".to_string(), "110".to_string());
    symbols.insert("ADM".to_string(), "111".to_string());

    symbols
}

pub fn rem_first_and_last(value: String) -> String {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_string()
}

pub fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}
