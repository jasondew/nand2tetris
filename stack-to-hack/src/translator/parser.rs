use crate::translator::instruction::*;

pub fn parse<S>(line: S) -> Instruction
where
    S: Into<String>,
{
    use Instruction::*;

    let line = line.into();
    let line = match line.split_once("//") {
        Some((instruction, _comment)) => instruction,
        None => line.as_str(),
    };

    match line.split_whitespace().collect::<Vec<&str>>()[..] {
        ["add"] => Add,
        ["sub"] => Subtract,
        ["neg"] => Negate,
        ["eq"] => Equal,
        ["gt"] => GreaterThan,
        ["lt"] => LessThan,
        ["and"] => And,
        ["or"] => Or,
        ["not"] => Not,
        ["push", segment, index_or_value] => {
            Push(parse_segment(segment), parse_number(index_or_value))
        }
        ["pop", segment, index_or_value] => {
            Pop(parse_segment(segment), parse_number(index_or_value))
        }
        ["label", name] => Label(name.into()),
        ["goto", name] => Goto(name.into()),
        ["if-goto", name] => IfGoto(name.into()),
        ["function", name, arity] => Function(name.into(), parse_number(arity)),
        ["call", name, arity] => Call(name.into(), parse_number(arity)),
        ["return"] => Return,
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
