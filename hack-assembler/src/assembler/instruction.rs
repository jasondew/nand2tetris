#[derive(Debug)]
pub enum Instruction {
    AInstruction(u16),
    CInstruction {
        destinations: Vec<Destination>,
        computation: Computation,
        jump: Option<JumpCondition>,
    },
}

#[derive(Debug)]
pub enum ParsedInstruction {
    AInstructionWithSymbol(String),
    AInstructionWithNumber(u16),
    CInstruction {
        destinations: Vec<Destination>,
        computation: Computation,
        jump: Option<JumpCondition>,
    },
    Label(String),
}

#[derive(Debug)]
pub enum Destination {
    Memory,
    DRegister,
    ARegister,
}

#[derive(Debug)]
pub enum Computation {
    Zero,
    One,
    NegativeOne,
    Identity(Destination),
    Not(Destination),
    Negative(Destination),
    PlusOne(Destination),
    MinusOne(Destination),
    DRegisterPlusARegister,
    DRegisterPlusMemory,
    DRegisterMinusARegister,
    DRegisterMinusMemory,
    ARegisterMinusDRegister,
    MemoryMinusDRegister,
    DRegisterAndARegister,
    DRegisterAndMemory,
    DRegisterOrARegister,
    DRegisterOrMemory,
}

#[derive(Debug)]
pub enum JumpCondition {
    JumpIfGreaterThan,
    JumpIfEqual,
    JumpIfGreaterThanOrEqual,
    JumpIfLessThan,
    JumpIfNotEqual,
    JumpIfLessThanOrEqual,
    JumpUnconditional,
}
