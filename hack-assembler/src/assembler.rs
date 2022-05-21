use std::collections::HashMap;

mod code;
mod instruction;
mod parser;

use crate::assembler::instruction::Instruction;
use crate::assembler::instruction::ParsedInstruction;

pub fn compile(contents: &String) -> Vec<String> {
    let parsed_instructions: Vec<ParsedInstruction> = contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| !(line.starts_with("//") || line.is_empty()))
        .map(|line| parser::parse(&line))
        .collect();

    let mut symbols: HashMap<String, u16> = HashMap::from([
        ("SP".to_owned(), 0),
        ("LCL".to_owned(), 1),
        ("ARG".to_owned(), 2),
        ("THIS".to_owned(), 3),
        ("THAT".to_owned(), 4),
        ("R0".to_owned(), 0),
        ("R1".to_owned(), 1),
        ("R2".to_owned(), 2),
        ("R3".to_owned(), 3),
        ("R4".to_owned(), 4),
        ("R5".to_owned(), 5),
        ("R6".to_owned(), 6),
        ("R7".to_owned(), 7),
        ("R8".to_owned(), 8),
        ("R9".to_owned(), 9),
        ("R10".to_owned(), 10),
        ("R11".to_owned(), 11),
        ("R12".to_owned(), 12),
        ("R13".to_owned(), 13),
        ("R14".to_owned(), 14),
        ("R15".to_owned(), 15),
        ("SCREEN".to_owned(), 16384),
        ("KBD".to_owned(), 24576),
    ]);

    let mut pc: u16 = 0;
    for parsed_instruction in parsed_instructions.iter() {
        match parsed_instruction {
            ParsedInstruction::Label(string) => {
                symbols.insert(string.clone(), pc);
            }
            _ => {
                pc += 1;
            }
        }
    }
    pc = 0;

    let mut next_symbol_value: u16 = 16;
    let mut instructions: Vec<Instruction> = Vec::new();
    for parsed_instruction in parsed_instructions {
        match parsed_instruction {
            ParsedInstruction::AInstructionWithNumber(value) => {
                instructions.push(Instruction::AInstruction(value));
                pc += 1
            }
            ParsedInstruction::AInstructionWithSymbol(string) => {
                if let Some(&value) = symbols.get(&string) {
                    instructions.push(Instruction::AInstruction(value));
                    pc += 1
                } else {
                    symbols.insert(string, next_symbol_value);
                    instructions
                        .push(Instruction::AInstruction(next_symbol_value));
                    next_symbol_value += 1;
                    pc += 1
                }
            }
            ParsedInstruction::CInstruction {
                destinations,
                computation,
                jump,
            } => {
                instructions.push(Instruction::CInstruction {
                    destinations,
                    computation,
                    jump,
                });
                pc += 1
            }
            ParsedInstruction::Label(_) => {}
        }
    }

    instructions
        .iter()
        .map(|line| code::encode(&line))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ignores_comments() {
        assert_eq!(
            Vec::<String>::new(),
            compile(&"// this is a comment\n// and another one".to_string())
        );
        assert_eq!(
            vec!["0000000000000000"],
            compile(&"@0  // end-of-line comment".to_string())
        );
    }

    #[test]
    fn test_ignores_blank_lines() {
        assert_eq!(Vec::<String>::new(), compile(&"  \n\n".to_string()));
    }

    #[test]
    fn test_a_instruction() {
        assert_eq!(vec!["0000000000000000"], compile(&"@0".to_string()));
        assert_eq!(vec!["0000000000000111"], compile(&"@7".to_string()));
        assert_eq!(vec!["0111111111111111"], compile(&"@32767".to_string()));
    }

    #[test]
    fn test_c_instruction() {
        assert_eq!(vec!["1111110000010000"], compile(&"D=M".to_string()));
        assert_eq!(vec!["1110000010010000"], compile(&"D=D+A".to_string()));
        assert_eq!(
            vec!["1111010101111010"],
            compile(&"AMD=D|M;JEQ".to_string())
        );
    }

    #[test]
    fn test_symbols() {
        let program = "\
        @R0
        M=1
        @LOOP
        0;JMP
        (LOOP)
        @DE_NOVO
        M=0
        ";
        assert_eq!(
            vec![
                "0000000000000000",
                "1110111111001000",
                "0000000000000100",
                "1110101010000111",
                "0000000000010000",
                "1110101010001000"
            ],
            compile(&program.to_string())
        );
    }
}
