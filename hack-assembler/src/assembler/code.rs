use crate::assembler::instruction::*;

pub fn encode(instruction: &Instruction) -> String {
    match instruction {
        Instruction::AInstruction(value) => format!("{:016b}", value),
        Instruction::CInstruction {
            destinations,
            computation,
            jump,
        } => {
            format!(
                "111{}{}{}",
                encode_computation(computation),
                encode_destinations(destinations),
                encode_jump(jump)
            )
        }
    }
}

fn encode_computation(computation: &Computation) -> &str {
    use Computation::*;
    use Destination::*;

    match computation {
        Zero => "0101010",
        One => "0111111",
        NegativeOne => "0111010",
        Identity(destination) => match destination {
            DRegister => "0001100",
            ARegister => "0110000",
            Memory => "1110000",
        },
        Not(destination) => match destination {
            DRegister => "0001101",
            ARegister => "0110001",
            Memory => "1110001",
        },
        Negative(destination) => match destination {
            DRegister => "0001111",
            ARegister => "0110011",
            Memory => "1110011",
        },
        PlusOne(destination) => match destination {
            DRegister => "0011111",
            ARegister => "0110111",
            Memory => "1110111",
        },
        MinusOne(destination) => match destination {
            DRegister => "0001110",
            ARegister => "0110010",
            Memory => "1110010",
        },
        DRegisterPlusARegister => "0000010",
        DRegisterPlusMemory => "1000010",
        DRegisterMinusARegister => "0010011",
        DRegisterMinusMemory => "1010011",
        ARegisterMinusDRegister => "0000111",
        MemoryMinusDRegister => "1000111",
        DRegisterAndARegister => "0000000",
        DRegisterAndMemory => "1000000",
        DRegisterOrARegister => "0010101",
        DRegisterOrMemory => "1010101",
    }
}

fn encode_destinations(destinations: &Vec<Destination>) -> String {
    use Destination::*;

    let value = destinations.iter().fold(0, |total, destination| {
        total
            + match destination {
                Memory => 1,
                DRegister => 2,
                ARegister => 4,
            }
    });
    format!("{:03b}", value)
}

fn encode_jump(jump: &Option<JumpCondition>) -> &str {
    use JumpCondition::*;

    match jump {
        None => "000",
        Some(JumpIfGreaterThan) => "001",
        Some(JumpIfEqual) => "010",
        Some(JumpIfGreaterThanOrEqual) => "011",
        Some(JumpIfLessThan) => "100",
        Some(JumpIfNotEqual) => "101",
        Some(JumpIfLessThanOrEqual) => "110",
        Some(JumpUnconditional) => "111",
    }
}
