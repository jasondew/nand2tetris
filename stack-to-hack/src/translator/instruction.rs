#[derive(Debug)]
pub enum Instruction {
    Push(Segment, u16),
    Pop(Segment, u16),
    Add,
    Subtract,
    Negate,
    Equal,
    GreaterThan,
    LessThan,
    And,
    Or,
    Not,
    Label(String),
    Goto(String),
    IfGoto(String),
    Function(String, u16),
    Call(String, u16),
    Return,
}

#[derive(Debug)]
pub enum Segment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}
