use crate::translator::instruction::Instruction::*;
use crate::translator::instruction::*;

pub fn to_hack(
    instruction: Instruction,
    line: &str,
    name: &str,
    id: &str,
) -> Vec<String> {
    //   let mut instructions = vec!["@32767".into(), format!("// {}", line)];
    let mut instructions = vec![format!("// {}", line)];

    match instruction {
        Add => {
            binary_operation(&mut instructions, &mut vec!["M=M+D".into()]);
            instructions.append(&mut increment_stack_pointer());
        }
        Subtract => {
            binary_operation(&mut instructions, &mut vec!["M=M-D".into()]);
            instructions.append(&mut increment_stack_pointer());
        }
        Negate => {
            instructions.append(&mut decrement_stack_pointer());
            instructions.append(&mut vec![
                "@SP".into(),
                "A=M".into(),
                "M=-M".into(),
            ]);
            instructions.append(&mut increment_stack_pointer());
        }
        Equal => {
            let equal_label = create_label("EQUAL", id);
            let done_label = create_label("DONE", id);

            binary_operation(&mut instructions, &mut vec!["D=M-D".into()]);
            instructions.append(&mut vec![
                format!("@{}", equal_label),
                "D;JEQ".into(),
                "D=0".into(),
                format!("@{}", done_label),
                "0;JMP".into(),
                format!("({})", equal_label),
                "D=-1".into(),
                format!("({})", done_label),
                "@SP".into(),
                "A=M".into(),
                "M=D".into(),
            ]);
            instructions.append(&mut increment_stack_pointer());
        }
        GreaterThan => {
            let gt_label = create_label("GT", id);
            let done_label = create_label("DONE", id);

            binary_operation(&mut instructions, &mut vec!["D=M-D".into()]);
            instructions.append(&mut vec![
                format!("@{}", gt_label),
                "D;JGT".into(),
                "D=0".into(),
                format!("@{}", done_label),
                "0;JMP".into(),
                format!("({})", gt_label),
                "D=-1".into(),
                format!("({})", done_label),
                "@SP".into(),
                "A=M".into(),
                "M=D".into(),
            ]);
            instructions.append(&mut increment_stack_pointer());
        }
        LessThan => {
            let lt_label = create_label("LT", id);
            let done_label = create_label("DONE", id);

            binary_operation(&mut instructions, &mut vec!["D=M-D".into()]);
            instructions.append(&mut vec![
                format!("@{}", lt_label),
                "D;JLT".into(),
                "D=0".into(),
                format!("@{}", done_label),
                "0;JMP".into(),
                format!("({})", lt_label),
                "D=-1".into(),
                format!("({})", done_label),
                "@SP".into(),
                "A=M".into(),
                "M=D".into(),
            ]);
            instructions.append(&mut increment_stack_pointer());
        }
        And => {
            binary_operation(&mut instructions, &mut vec!["M=D&M".into()]);
            instructions.append(&mut increment_stack_pointer());
        }
        Or => {
            binary_operation(&mut instructions, &mut vec!["M=D|M".into()]);
            instructions.append(&mut increment_stack_pointer());
        }
        Not => {
            instructions.append(&mut decrement_stack_pointer());
            instructions.append(&mut vec![
                "@SP".into(),
                "A=M".into(),
                "M=!M".into(),
            ]);
            instructions.append(&mut increment_stack_pointer());
        }
        Push(segment, offset) => {
            instructions
                .append(&mut load_d_from_segment(segment, offset, name));
            instructions.append(&mut push_d_onto_stack());
            instructions.append(&mut increment_stack_pointer());
        }
        Pop(segment, offset) => {
            instructions
                .append(&mut calculate_address_into_r13(segment, offset, name));
            instructions.append(&mut pop_stack_into_r13_pointer());
        }
    }

    instructions
}

fn create_label(name: &str, id: &str) -> String {
    format!("{}-{}", name, id)
}

fn binary_operation(
    instructions: &mut Vec<String>,
    operation: &mut Vec<String>,
) {
    instructions.append(&mut pop_stack_into_d());
    instructions.append(&mut pop_stack_address_into_a());
    instructions.append(operation);
}

fn pop_stack_into_d() -> Vec<String> {
    vec!["@SP".into(), "M=M-1".into(), "A=M".into(), "D=M".into()]
}

fn pop_stack_address_into_a() -> Vec<String> {
    vec!["@SP".into(), "M=M-1".into(), "A=M".into()]
}

fn push_d_onto_stack() -> Vec<String> {
    vec!["@SP".into(), "A=M".into(), "M=D".into()]
}

fn load_d_from_segment(
    segment: Segment,
    offset: u16,
    name: &str,
) -> Vec<String> {
    use Segment::*;

    match segment {
        Constant => vec![format!("@{}", offset), "D=A".into()],
        Temp => vec![format!("@R{}", offset + 5), "D=M".into()],
        Pointer => match offset {
            0 => vec!["@THIS".into(), "D=M".into()],
            1 => vec!["@THAT".into(), "D=M".into()],
            _ => panic!("invalid pointer offset: {}", offset),
        },
        Local | Argument | This | That => {
            let mut instructions =
                calculate_address_into_r13(segment, offset, name);
            instructions.append(&mut vec![
                "@R13".into(),
                "A=M".into(),
                "D=M".into(),
            ]);
            instructions
        }
        Static => vec![format!("@{}.{}", name, offset), "D=M".into()],
    }
}

fn increment_stack_pointer() -> Vec<String> {
    vec!["@SP".into(), "M=M+1".into()]
}

fn decrement_stack_pointer() -> Vec<String> {
    vec!["@SP".into(), "M=M-1".into()]
}

fn pop_stack_into_r13_pointer() -> Vec<String> {
    let mut instructions = decrement_stack_pointer();
    instructions.append(&mut vec![
        "A=M".into(),
        "D=M".into(),
        "@R13".into(),
        "A=M".into(),
        "M=D".into(),
    ]);
    instructions
}

fn calculate_address_into_r13(
    segment: Segment,
    offset: u16,
    name: &str,
) -> Vec<String> {
    use Segment::*;

    match segment {
        Constant => panic!(
            "unexpected error attempted to calculate address for {:?}",
            segment
        ),
        Temp => vec![
            format!("@R{}", offset + 5),
            "D=A".into(),
            "@R13".into(),
            "M=D".into(),
        ],
        Pointer => match offset {
            0 => {
                vec!["@THIS".into(), "D=A".into(), "@R13".into(), "M=D".into()]
            }
            1 => {
                vec!["@THAT".into(), "D=A".into(), "@R13".into(), "M=D".into()]
            }
            _ => panic!("invalid pointer offset: {}", offset),
        },
        Static => vec![
            format!("@{}.{}", name, offset),
            "D=A".into(),
            "@R13".into(),
            "M=D".into(),
        ],
        _ => vec![
            load_segment_base(segment),
            "D=M".into(),
            load_address(offset),
            "D=A+D".into(),
            "@R13".into(),
            "M=D".into(),
        ],
    }
}

fn load_segment_base(segment: Segment) -> String {
    use Segment::*;

    match segment {
        Argument => "@ARG",
        Local => "@LCL",
        This => "@THIS",
        That => "@THAT",
        _ => panic!("invalid segment base: {:?}", segment),
    }
    .into()
}

fn load_address(value: u16) -> String {
    format!("@{}", value)
}
