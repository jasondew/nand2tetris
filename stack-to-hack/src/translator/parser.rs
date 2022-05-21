use crate::translator::instruction::*;

pub fn parse(line: &str) -> Instruction {
    match line.split_whitespace().collect::<Vec<&str>>()[..] {
        ["add"] => Instruction::Add,
        ["sub"] => Instruction::Subtract,
        ["neg"] => Instruction::Negate,
        ["eq"] => Instruction::Equal,
        ["gt"] => Instruction::GreaterThan,
        ["lt"] => Instruction::LessThan,
        ["and"] => Instruction::And,
        ["or"] => Instruction::Or,
        ["not"] => Instruction::Not,
        ["push", segment, index_or_value] => Instruction::Push(
            parse_segment(segment),
            parse_number(index_or_value),
        ),
        ["pop", segment, index_or_value] => Instruction::Pop(
            parse_segment(segment),
            parse_number(index_or_value),
        ),
        _ => panic!("unknown instruction: {}", line),
    }
}

fn parse_segment(segment: &str) -> Segment {
    match segment {
        "argument" => Segment::Argument,
        "local" => Segment::Local,
        "static" => Segment::Static,
        "constant" => Segment::Constant,
        "this" => Segment::This,
        "that" => Segment::That,
        "pointer" => Segment::Pointer,
        "temp" => Segment::Temp,
        _ => panic!("unknown segment: {}", segment),
    }
}

fn parse_number(string: &str) -> u16 {
    match string.parse::<u16>() {
        Ok(value) => value,
        Err(_) => panic!("invalid number: {}", string),
    }
}
