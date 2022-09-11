use std::env;
use std::fs::File;
use std::io::prelude::*;

use std::io::Error;

mod code;
mod line;
mod parser;
mod symbols;
mod utils;

use code::Code;
use parser::Parser;

use crate::line::InstructionType;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    // Main logic to run program
    if let Some(filename) = args.get(1) {
        // let mut parser = Parser::new("test.txt".to_string());
        let mut parser = Parser::new(filename.clone());

        let mut symbols = parser.read_lines()?;
        parser.print_source();

        let mut code = Code::new(&mut symbols);
        let mut instructions = Vec::new();

        // build vec of instructions
        for source in parser.source {
            let instruction = code.instruction_from_source(source);
            if instruction.inst_type != InstructionType::L {
                instructions.push(instruction);
            }
        }

        // Write lines to file
        let mut file = File::create("out.hack")?;
        for ins in instructions {
            // println!("{}", &ins);
            writeln!(file, "{}", ins.binary_str)?
        }
    } else {
        println!("There was an error parsing the args, please prove filename")
    }

    Ok(())
}
