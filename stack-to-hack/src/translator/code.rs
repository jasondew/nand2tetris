use crate::translator::instruction::Instruction::*;
use crate::translator::instruction::*;
use std::cell::RefCell;

thread_local!( static CALL_COUNT: RefCell<u16> = RefCell::new(0));
thread_local!( static CURRENT_FUNCTION: RefCell<String> = RefCell::new("".into()));

pub fn bootstrap() -> Vec<String> {
    let mut instructions =
        to_strings(vec!["// SP = 256", "@256", "D=A", "@SP", "M=D"]);
    instructions.append(&mut to_hack(
        Call("Sys.init".into(), 0),
        "call Sys.init",
        "Sys",
    ));
    instructions
}

pub fn to_hack<S>(
    instruction: Instruction,
    line: S,
    module_name: S,
) -> Vec<String>
where
    S: AsRef<str>,
{
    //    let mut instructions = vec!["@32767".into(), format!("// {}", line.as_ref())];
    let mut instructions = vec![format!("// {}", line.as_ref())];

    match instruction {
        Add => {
            binary_operation(&mut instructions, "M=M+D");
            instructions.append(&mut increment_stack_pointer());
        }
        Subtract => {
            binary_operation(&mut instructions, "M=M-D");
            instructions.append(&mut increment_stack_pointer());
        }
        Negate => {
            instructions.append(&mut decrement_stack_pointer());
            instructions.append(&mut to_strings(vec!["@SP", "A=M", "M=-M"]));
            instructions.append(&mut increment_stack_pointer());
        }
        Equal => {
            binary_operation(&mut instructions, "D=M-D");
            instructions.append(&mut vec![
                load_label_address("EQUAL"),
                "D;JEQ".into(),
                "D=0".into(),
                load_label_address("DONE"),
                unconditional_jump(),
                create_label("EQUAL"),
                "D=-1".into(),
                create_label("DONE"),
                "@SP".into(),
                "A=M".into(),
                "M=D".into(),
            ]);
            instructions.append(&mut increment_stack_pointer());
        }
        GreaterThan => {
            binary_operation(&mut instructions, "D=M-D");
            instructions.append(&mut vec![
                load_label_address("GT"),
                "D;JGT".into(),
                "D=0".into(),
                load_label_address("DONE"),
                unconditional_jump(),
                create_label("GT"),
                "D=-1".into(),
                create_label("DONE"),
                "@SP".into(),
                "A=M".into(),
                "M=D".into(),
            ]);
            instructions.append(&mut increment_stack_pointer());
        }
        LessThan => {
            binary_operation(&mut instructions, "D=M-D");
            instructions.append(&mut vec![
                load_label_address("LT"),
                "D;JLT".into(),
                "D=0".into(),
                load_label_address("DONE"),
                unconditional_jump(),
                create_label("LT"),
                "D=-1".into(),
                create_label("DONE"),
                "@SP".into(),
                "A=M".into(),
                "M=D".into(),
            ]);
            instructions.append(&mut increment_stack_pointer());
        }
        And => {
            binary_operation(&mut instructions, "M=D&M");
            instructions.append(&mut increment_stack_pointer());
        }
        Or => {
            binary_operation(&mut instructions, "M=D|M");
            instructions.append(&mut increment_stack_pointer());
        }
        Not => {
            instructions.append(&mut decrement_stack_pointer());
            instructions.append(&mut to_strings(vec!["@SP", "A=M", "M=!M"]));
            instructions.append(&mut increment_stack_pointer());
        }
        Push(segment, offset) => {
            instructions.append(&mut load_d_from_segment(
                segment,
                offset,
                module_name.as_ref(),
            ));
            instructions.append(&mut push_d_onto_stack());
        }
        Pop(segment, offset) => {
            instructions.append(&mut calculate_address_into_r13(
                segment,
                offset,
                module_name.as_ref(),
            ));
            instructions.append(&mut pop_stack_into_r13_pointer());
        }
        Label(label) => {
            instructions.append(&mut vec![create_label(label.as_ref())])
        }
        Goto(label) => instructions.append(&mut vec![
            load_label_address(label.as_ref()),
            unconditional_jump(),
        ]),
        IfGoto(label) => {
            instructions.append(&mut pop_stack_into_d());
            instructions.append(&mut vec![
                load_label_address(label.as_ref()),
                jump_if_d_is_not_zero(),
            ]);
        }
        Function(name, arity) => {
            CURRENT_FUNCTION.with(|function| {
                *function.borrow_mut() = name.clone();
            });
            instructions
                .append(&mut vec![create_function_label(name.as_ref())]);
            for _ in 1..=arity {
                instructions.append(&mut vec![
                    "@SP".into(),
                    "A=M".into(),
                    "M=0".into(),
                ]);
                instructions.append(&mut increment_stack_pointer());
            }
        }
        Call(name, arity) => {
            let return_label = format_function_return_label(name.as_ref());
            // five values: return location, LCL, ARG, THIS, THAT
            let stack_size = arity + 5;

            // push retAddr
            instructions.append(&mut push_label_onto_stack(return_label));
            // push LCL
            instructions.append(&mut push_pointer_onto_stack("LCL".into()));
            // push ARG
            instructions.append(&mut push_pointer_onto_stack("ARG".into()));
            // push THIS
            instructions.append(&mut push_pointer_onto_stack("THIS".into()));
            // push THAT
            instructions.append(&mut push_pointer_onto_stack("THAT".into()));
            // ARG=SP-5-nArgs
            instructions.append(&mut vec![
                "@SP".into(),
                "D=M".into(),
                format!("@{}", stack_size),
                "D=D-A".into(),
                "@ARG".into(),
                "M=D".into(),
            ]);
            // LCL=SP
            instructions
                .append(&mut to_strings(vec!["@SP", "D=M", "@LCL", "M=D"]));
            // goto f
            instructions
                .append(&mut vec![format!("@{}", name), "0;JMP".into()]);
            // return address label
            instructions
                .append(&mut vec![create_function_return_label(name.as_ref())]);

            CALL_COUNT.with(|call_count| {
                *call_count.borrow_mut() += 1;
            });
        }
        Return => {
            instructions.append(&mut save_lcl_into_r14());
            instructions.append(&mut save_return_address_into_r15());
            instructions.append(&mut reset_stack_with_return_value());
            instructions.append(&mut restore_frame_at_r14());
            instructions.append(&mut jump_back_to_caller());
        }
    }

    instructions
}

fn create_label(name: &str) -> String {
    format!("({})", format_label(name))
}

fn create_function_label(name: &str) -> String {
    format!("({})", name)
}

fn create_function_return_label(name: &str) -> String {
    format!("({})", format_function_return_label(name))
}

fn format_label(name: &str) -> String {
    CURRENT_FUNCTION.with(|function| {
        if function.borrow().is_empty() {
            name.to_owned()
        } else {
            format!("{}${}", function.borrow(), name)
        }
    })
}

fn format_function_return_label(name: &str) -> String {
    CALL_COUNT
        .with(|call_count| format!("{}$return.{}", name, call_count.borrow()))
}

fn binary_operation<S>(instructions: &mut Vec<String>, operation: S)
where
    S: AsRef<str>,
{
    instructions.append(&mut pop_stack_into_d());
    instructions.append(&mut pop_stack_address_into_a());
    instructions.push(operation.as_ref().to_owned());
}

fn jump_if_d_is_not_zero() -> String {
    "D;JNE".into()
}

fn unconditional_jump() -> String {
    "0;JMP".into()
}

fn pop_stack_into_d() -> Vec<String> {
    to_strings(vec!["@SP", "M=M-1", "A=M", "D=M"])
}

fn pop_stack_address_into_a() -> Vec<String> {
    to_strings(vec!["@SP", "M=M-1", "A=M"])
}

fn push_d_onto_stack() -> Vec<String> {
    to_strings(vec!["@SP", "A=M", "M=D", "@SP", "M=M+1"])
}

fn push_label_onto_stack(label: String) -> Vec<String> {
    let mut instructions = vec![format!("@{}", label), "D=A".into()];
    instructions.append(&mut push_d_onto_stack());
    instructions
}

fn push_pointer_onto_stack(name: String) -> Vec<String> {
    let mut instructions = vec![format!("@{}", name), "D=M".into()];
    instructions.append(&mut push_d_onto_stack());
    instructions
}

fn load_d_from_segment<S>(
    segment: Segment,
    offset: u16,
    module_name: S,
) -> Vec<String>
where
    S: AsRef<str>,
{
    use Segment::*;

    match segment {
        Constant => vec![load_address(offset), "D=A".into()],
        Temp => vec![format!("@R{}", offset + 5), "D=M".into()],
        Pointer => match offset {
            0 => vec!["@THIS".into(), "D=M".into()],
            1 => vec!["@THAT".into(), "D=M".into()],
            _ => panic!("invalid pointer offset: {}", offset),
        },
        Local | Argument | This | That => {
            let mut instructions =
                calculate_address_into_r13(segment, offset, module_name);
            instructions.append(&mut vec![
                "@R13".into(),
                "A=M".into(),
                "D=M".into(),
            ]);
            instructions
        }
        Static => vec![
            load_static_variable_address(offset, module_name.as_ref()),
            "D=M".into(),
        ],
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

fn calculate_address_into_r13<S>(
    segment: Segment,
    offset: u16,
    module_name: S,
) -> Vec<String>
where
    S: AsRef<str>,
{
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
            load_static_variable_address(offset, module_name.as_ref()),
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

fn save_lcl_into_r14() -> Vec<String> {
    // frame = R14 = LCL
    to_strings(vec!["@LCL", "D=M", "@R14", "M=D"])
}

fn save_return_address_into_r15() -> Vec<String> {
    // assumed D = frame
    // retAddr = R15 = *(frame - 5)
    to_strings(vec!["@5", "A=D-A", "D=M", "@R15", "M=D"])
}

fn reset_stack_with_return_value() -> Vec<String> {
    to_strings(vec![
        // *ARG = pop()
        "@SP", "M=M-1", "A=M", "D=M", "@ARG", "A=M", "M=D",
        // SP = ARG + 1
        "@ARG", "D=M", "@SP", "M=D+1",
    ])
}

fn restore_frame_at_r14() -> Vec<String> {
    to_strings(vec![
        // THAT = *(frame - 1)
        "@R14", "M=M-1", "A=M", "D=M", "@THAT", "M=D",
        // THIS = *(frame - 2)
        "@R14", "M=M-1", "A=M", "D=M", "@THIS", "M=D",
        // ARG = *(frame - 3)
        "@R14", "M=M-1", "A=M", "D=M", "@ARG", "M=D",
        // LCL = *(frame - 4)
        "@R14", "M=M-1", "A=M", "D=M", "@LCL", "M=D",
    ])
}

fn jump_back_to_caller() -> Vec<String> {
    // goto retAddr = R15
    to_strings(vec!["@R15", "A=M", "0;JMP"])
}

fn load_segment_base(segment: Segment) -> String {
    use Segment::*;

    match segment {
        Argument => "@ARG".into(),
        Local => "@LCL".into(),
        This => "@THIS".into(),
        That => "@THAT".into(),
        _ => panic!("invalid segment base: {:?}", segment),
    }
}

fn load_address(value: u16) -> String {
    format!("@{}", value)
}

fn load_static_variable_address(number: u16, module_name: &str) -> String {
    format!("@{}.{}", module_name, number)
}

fn load_label_address(name: &str) -> String {
    format!("@{}", format_label(name))
}

fn to_strings<S>(strings: Vec<S>) -> Vec<String>
where
    S: Into<String>,
{
    strings.into_iter().map(|s| s.into()).collect()
}
