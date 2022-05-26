mod code;
mod instruction;
mod parser;

pub fn bootstrap() -> Vec<String> {
    code::bootstrap()
}

pub fn translate<S>(line: S, name: S) -> Vec<String>
where
    S: Into<String>,
{
    let line = line.into();
    code::to_hack(parser::parse(line.clone()), line, name.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ignores_inline_comments() {
        assert_eq!(
            vec![
                "// push constant 0  // end-of-line comment",
                "@0",
                "D=A",
                "@SP",
                "A=M",
                "M=D",
                "@SP",
                "M=M+1"
            ],
            translate("push constant 0  // end-of-line comment", "Foo")
        );
    }

    #[test]
    fn test_push_constant() {
        assert_eq!(
            vec![
                "// push constant 7",
                "@7",
                "D=A",
                "@SP",
                "A=M",
                "M=D",
                "@SP",
                "M=M+1"
            ],
            translate("push constant 7", "Foo")
        );
    }

    #[test]
    fn test_pop_local() {
        assert_eq!(
            vec![
                "// pop local 2",
                "@LCL",
                "D=M",
                "@2",
                "D=A+D",
                "@R13",
                "M=D",
                "@SP",
                "M=M-1",
                "A=M",
                "D=M",
                "@R13",
                "A=M",
                "M=D"
            ],
            translate("pop local 2", "Foo")
        );
    }

    #[test]
    fn test_push_pointer_zero() {
        assert_eq!(
            vec![
                "// push pointer 0",
                "@THIS",
                "D=M",
                "@SP",
                "A=M",
                "M=D",
                "@SP",
                "M=M+1"
            ],
            translate("push pointer 0", "Foo")
        );
    }

    #[test]
    fn test_pop_pointer_one() {
        assert_eq!(
            vec![
                "// pop pointer 1",
                "@THAT",
                "D=A",
                "@R13",
                "M=D",
                "@SP",
                "M=M-1",
                "A=M",
                "D=M",
                "@R13",
                "A=M",
                "M=D",
            ],
            translate("pop pointer 1", "Foo")
        );
    }

    #[test]
    fn test_push_static() {
        assert_eq!(
            vec![
                "// push static 8",
                "@Foo.8",
                "D=M",
                "@SP",
                "A=M",
                "M=D",
                "@SP",
                "M=M+1"
            ],
            translate("push static 8", "Foo")
        );
    }

    #[test]
    fn test_pop_static() {
        assert_eq!(
            vec![
                "// pop static 0",
                "@Foo.0",
                "D=A",
                "@R13",
                "M=D",
                "@SP",
                "M=M-1",
                "A=M",
                "D=M",
                "@R13",
                "A=M",
                "M=D"
            ],
            translate("pop static 0", "Foo")
        );
    }

    #[test]
    fn test_add() {
        assert_eq!(
            vec![
                "// add", "@SP", "M=M-1", "A=M", "D=M", "@SP", "M=M-1", "A=M",
                "M=M+D", "@SP", "M=M+1"
            ],
            translate("add", "Foo")
        );
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            vec![
                "// sub", "@SP", "M=M-1", "A=M", "D=M", "@SP", "M=M-1", "A=M",
                "M=M-D", "@SP", "M=M+1"
            ],
            translate("sub", "Foo")
        );
    }

    #[test]
    fn test_neg() {
        assert_eq!(
            vec![
                "// neg", "@SP", "M=M-1", "@SP", "A=M", "M=-M", "@SP", "M=M+1"
            ],
            translate("neg", "Foo")
        );
    }

    #[test]
    fn test_eq() {
        assert_eq!(
            vec![
                "// eq", "@SP", "M=M-1", "A=M", "D=M", "@SP", "M=M-1", "A=M",
                "D=M-D", "@EQUAL", "D;JEQ", "D=0", "@DONE", "0;JMP", "(EQUAL)",
                "D=-1", "(DONE)", "@SP", "A=M", "M=D", "@SP", "M=M+1"
            ],
            translate("eq", "Foo")
        );
    }

    #[test]
    fn test_gt() {
        assert_eq!(
            vec![
                "// gt", "@SP", "M=M-1", "A=M", "D=M", "@SP", "M=M-1", "A=M",
                "D=M-D", "@GT", "D;JGT", "D=0", "@DONE", "0;JMP", "(GT)",
                "D=-1", "(DONE)", "@SP", "A=M", "M=D", "@SP", "M=M+1"
            ],
            translate("gt", "Foo")
        );
    }

    #[test]
    fn test_lt() {
        assert_eq!(
            vec![
                "// lt", "@SP", "M=M-1", "A=M", "D=M", "@SP", "M=M-1", "A=M",
                "D=M-D", "@LT", "D;JLT", "D=0", "@DONE", "0;JMP", "(LT)",
                "D=-1", "(DONE)", "@SP", "A=M", "M=D", "@SP", "M=M+1"
            ],
            translate("lt", "Foo")
        );
    }

    #[test]
    fn test_and() {
        assert_eq!(
            vec![
                "// and", "@SP", "M=M-1", "A=M", "D=M", "@SP", "M=M-1", "A=M",
                "M=D&M", "@SP", "M=M+1"
            ],
            translate("and", "Foo")
        );
    }

    #[test]
    fn test_or() {
        assert_eq!(
            vec![
                "// or", "@SP", "M=M-1", "A=M", "D=M", "@SP", "M=M-1", "A=M",
                "M=D|M", "@SP", "M=M+1"
            ],
            translate("or", "Foo")
        );
    }

    #[test]
    fn test_not() {
        assert_eq!(
            vec![
                "// not", "@SP", "M=M-1", "@SP", "A=M", "M=!M", "@SP", "M=M+1"
            ],
            translate("not", "Foo")
        );
    }

    #[test]
    fn test_label() {
        assert_eq!(
            vec!["// label LOOP_START", "(LOOP_START)"],
            translate("label LOOP_START", "Foo")
        );
    }

    #[test]
    fn test_goto() {
        assert_eq!(
            vec!["// goto LOOP_START", "@LOOP_START", "0;JMP"],
            translate("goto LOOP_START", "Foo")
        );
    }

    #[test]
    fn test_if_goto() {
        assert_eq!(
            vec![
                "// if-goto LOOP_START",
                "@SP",
                "M=M-1",
                "A=M",
                "D=M",
                "@LOOP_START",
                "D;JNE"
            ],
            translate("if-goto LOOP_START", "Foo")
        );
    }

    #[test]
    fn test_function() {
        assert_eq!(
            vec![
                "// function Foo.bar 3",
                "(Foo.bar)",
                "@SP",
                "A=M",
                "M=0",
                "@SP",
                "M=M+1",
                "@SP",
                "A=M",
                "M=0",
                "@SP",
                "M=M+1",
                "@SP",
                "A=M",
                "M=0",
                "@SP",
                "M=M+1",
            ],
            translate("function Foo.bar 3", "Foo")
        );
    }

    #[test]
    fn test_return() {
        assert_eq!(
            vec![
                "// return",
                // frame = LCL
                "@LCL",
                "D=M",
                "@R14",
                "M=D",
                // retAddr = *(frame - 5)
                "@5",
                "A=D-A",
                "D=M",
                "@R15",
                "M=D",
                // *ARG = pop()
                "@SP",
                "M=M-1",
                "A=M",
                "D=M",
                "@ARG",
                "A=M",
                "M=D",
                // SP = ARG + 1
                "@ARG",
                "D=M",
                "@SP",
                "M=D+1",
                // THAT = *(frame - 1)
                "@R14",
                "M=M-1",
                "A=M",
                "D=M",
                "@THAT",
                "M=D",
                // THIS = *(frame - 2)
                "@R14",
                "M=M-1",
                "A=M",
                "D=M",
                "@THIS",
                "M=D",
                // ARG = *(frame - 3)
                "@R14",
                "M=M-1",
                "A=M",
                "D=M",
                "@ARG",
                "M=D",
                // LCL = *(frame - 4)
                "@R14",
                "M=M-1",
                "A=M",
                "D=M",
                "@LCL",
                "M=D",
                // goto retAddr
                "@R15",
                "A=M",
                "0;JMP",
            ],
            translate("return", "Foo")
        );
    }

    #[test]
    fn test_call() {
        assert_eq!(
            vec![
                "// call Foo.bar 3",
                // push retAddr
                "@Foo.bar$return.0",
                "D=A",
                "@SP",
                "A=M",
                "M=D",
                "@SP",
                "M=M+1",
                // push LCL"
                "@LCL",
                "D=M",
                "@SP",
                "A=M",
                "M=D",
                "@SP",
                "M=M+1",
                // push ARG"
                "@ARG",
                "D=M",
                "@SP",
                "A=M",
                "M=D",
                "@SP",
                "M=M+1",
                // push THIS"
                "@THIS",
                "D=M",
                "@SP",
                "A=M",
                "M=D",
                "@SP",
                "M=M+1",
                // push THAT"
                "@THAT",
                "D=M",
                "@SP",
                "A=M",
                "M=D",
                "@SP",
                "M=M+1",
                // ARG=SP-5-nArgs"
                "@SP",
                "D=M",
                "@8",
                "D=D-A",
                "@ARG",
                "M=D",
                // LCL=SP"
                "@SP",
                "D=M",
                "@LCL",
                "M=D",
                // goto f"
                "@Foo.bar",
                "0;JMP",
                // return address label"
                "(Foo.bar$return.0)"
            ],
            translate("call Foo.bar 3", "Foo")
        );
    }
}
