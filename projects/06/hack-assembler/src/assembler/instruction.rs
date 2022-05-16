pub enum Instruction {
    AInstruction(u16),
    CInstruction {
        destinations: Vec<Destination>,
        computation: Computation,
        jump: Option<JumpCondition>,
    },
}

pub enum Destination {
    Memory,
    DRegister,
    ARegister,
}

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

pub enum JumpCondition {
    JumpIfGreaterThan,
    JumpIfEqual,
    JumpIfGreaterThanOrEqual,
    JumpIfLessThan,
    JumpIfNotEqual,
    JumpIfLessThanOrEqual,
    JumpUnconditional,
}
