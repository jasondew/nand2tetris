use crate::assembler::instruction::*;

pub fn parse(input: &str) -> ParsedInstruction {
    let line = input.replace(" ", "");
    let line = match line.split_once("//") {
        Some((instruction, _comment)) => instruction,
        None => line.as_str(),
    };

    if line.starts_with("(") {
        let parens: &[_] = &['(', ')'];
        ParsedInstruction::Label(line.trim_matches(parens).to_owned())
    } else if let Some(number_string) = line.strip_prefix("@") {
        parse_a_instruction(number_string)
    } else {
        if let Some(instruction) = parse_c_instruction(&line) {
            instruction
        } else {
            panic!("invalid C instruction: {}", line)
        }
    }
}

fn parse_a_instruction(string: &str) -> ParsedInstruction {
    if let Ok(number) = string.parse::<u16>() {
        ParsedInstruction::AInstructionWithNumber(number)
    } else {
        ParsedInstruction::AInstructionWithSymbol(string.to_owned())
    }
}

fn parse_c_instruction(line: &str) -> Option<ParsedInstruction> {
    let (destinations_and_computation, jump) = match line.split_once(';') {
        Some((rest, jump)) => (rest, jump),
        None => (line, ""),
    };

    let (destinations, computation) =
        match destinations_and_computation.split_once('=') {
            Some((destinations, computation)) => (destinations, computation),
            None => ("", destinations_and_computation),
        };
    Some(ParsedInstruction::CInstruction {
        computation: parse_computation(computation),
        destinations: parse_destinations(destinations),
        jump: parse_jump(jump),
    })
}

fn parse_computation(string: &str) -> Computation {
    use Computation::*;
    use Destination::*;

    match string {
        "0" => Zero,
        "1" => One,
        "-1" => NegativeOne,
        "D" => Identity(DRegister),
        "A" => Identity(ARegister),
        "M" => Identity(Memory),
        "!D" => Not(DRegister),
        "!A" => Not(ARegister),
        "!M" => Not(Memory),
        "-D" => Negative(DRegister),
        "-A" => Negative(ARegister),
        "-M" => Negative(Memory),
        "D+1" => PlusOne(DRegister),
        "A+1" => PlusOne(ARegister),
        "M+1" => PlusOne(Memory),
        "D-1" => MinusOne(DRegister),
        "A-1" => MinusOne(ARegister),
        "M-1" => MinusOne(Memory),
        "D+A" => DRegisterPlusARegister,
        "D+M" => DRegisterPlusMemory,
        "D-A" => DRegisterMinusARegister,
        "D-M" => DRegisterMinusMemory,
        "A-D" => ARegisterMinusDRegister,
        "M-D" => MemoryMinusDRegister,
        "D&A" => DRegisterAndARegister,
        "D&M" => DRegisterAndMemory,
        "D|A" => DRegisterOrARegister,
        "D|M" => DRegisterOrMemory,
        _ => panic!("unknown computation: {}", string),
    }
}

fn parse_destinations(string: &str) -> Vec<Destination> {
    use Destination::*;

    string
        .chars()
        .map(|ch| match ch {
            'D' => DRegister,
            'A' => ARegister,
            'M' => Memory,
            _ => panic!("unknown destination: {}", ch),
        })
        .collect()
}

fn parse_jump(string: &str) -> Option<JumpCondition> {
    use JumpCondition::*;

    match string {
        "" => None,
        "JGT" => Some(JumpIfGreaterThan),
        "JEQ" => Some(JumpIfEqual),
        "JGE" => Some(JumpIfGreaterThanOrEqual),
        "JLT" => Some(JumpIfLessThan),
        "JNE" => Some(JumpIfNotEqual),
        "JLE" => Some(JumpIfLessThanOrEqual),
        "JMP" => Some(JumpUnconditional),
        _ => panic!("unknown jump: {}", string),
    }
}
