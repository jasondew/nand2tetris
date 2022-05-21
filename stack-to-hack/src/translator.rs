mod code;
mod instruction;
mod parser;

pub fn translate(line: &str, name: &str) -> Vec<String> {
    let id = nanoid::nanoid!(4);
    translate_with_id(line, name, id.as_str())
}

fn translate_with_id(line: &str, name: &str, id: &str) -> Vec<String> {
    code::to_hack(parser::parse(line), line, name, id)
}

#[cfg(test)]
mod tests {
    use super::*;

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
            translate(&"push constant 7".to_string(), "Foo")
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
            translate(&"pop local 2".to_string(), "Foo")
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
            translate(&"push pointer 0".to_string(), "Foo")
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
            translate(&"pop pointer 1".to_string(), "Foo")
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
            translate(&"push static 8".to_string(), "Foo")
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
            translate(&"pop static 0".to_string(), "Foo")
        );
    }

    #[test]
    fn test_add() {
        assert_eq!(
            vec![
                "// add", "@SP", "M=M-1", "A=M", "D=M", "@SP", "M=M-1", "A=M",
                "M=M+D", "@SP", "M=M+1"
            ],
            translate(&"add".to_string(), "Foo")
        );
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            vec![
                "// sub", "@SP", "M=M-1", "A=M", "D=M", "@SP", "M=M-1", "A=M",
                "M=M-D", "@SP", "M=M+1"
            ],
            translate(&"sub".to_string(), "Foo")
        );
    }

    #[test]
    fn test_neg() {
        assert_eq!(
            vec![
                "// neg", "@SP", "M=M-1", "@SP", "A=M", "M=-M", "@SP", "M=M+1"
            ],
            translate(&"neg".to_string(), "Foo")
        );
    }

    #[test]
    fn test_eq() {
        assert_eq!(
            vec![
                "// eq",
                "@SP",
                "M=M-1",
                "A=M",
                "D=M",
                "@SP",
                "M=M-1",
                "A=M",
                "D=M-D",
                "@EQUAL-id",
                "D;JEQ",
                "D=0",
                "@DONE-id",
                "0;JMP",
                "(EQUAL-id)",
                "D=-1",
                "(DONE-id)",
                "@SP",
                "A=M",
                "M=D",
                "@SP",
                "M=M+1"
            ],
            translate_with_id(&"eq".to_string(), "Foo", "id")
        );
    }

    #[test]
    fn test_gt() {
        assert_eq!(
            vec![
                "// gt",
                "@SP",
                "M=M-1",
                "A=M",
                "D=M",
                "@SP",
                "M=M-1",
                "A=M",
                "D=M-D",
                "@GT-id",
                "D;JGT",
                "D=0",
                "@DONE-id",
                "0;JMP",
                "(GT-id)",
                "D=-1",
                "(DONE-id)",
                "@SP",
                "A=M",
                "M=D",
                "@SP",
                "M=M+1"
            ],
            translate_with_id(&"gt".to_string(), "Foo", "id")
        );
    }

    #[test]
    fn test_lt() {
        assert_eq!(
            vec![
                "// lt",
                "@SP",
                "M=M-1",
                "A=M",
                "D=M",
                "@SP",
                "M=M-1",
                "A=M",
                "D=M-D",
                "@LT-id",
                "D;JLT",
                "D=0",
                "@DONE-id",
                "0;JMP",
                "(LT-id)",
                "D=-1",
                "(DONE-id)",
                "@SP",
                "A=M",
                "M=D",
                "@SP",
                "M=M+1"
            ],
            translate_with_id(&"lt".to_string(), "Foo", "id")
        );
    }

    #[test]
    fn test_and() {
        assert_eq!(
            vec![
                "// and", "@SP", "M=M-1", "A=M", "D=M", "@SP", "M=M-1", "A=M",
                "M=D&M", "@SP", "M=M+1"
            ],
            translate(&"and".to_string(), "Foo")
        );
    }

    #[test]
    fn test_or() {
        assert_eq!(
            vec![
                "// or", "@SP", "M=M-1", "A=M", "D=M", "@SP", "M=M-1", "A=M",
                "M=D|M", "@SP", "M=M+1"
            ],
            translate(&"or".to_string(), "Foo")
        );
    }

    #[test]
    fn test_not() {
        assert_eq!(
            vec![
                "// not", "@SP", "M=M-1", "@SP", "A=M", "M=!M", "@SP", "M=M+1"
            ],
            translate(&"not".to_string(), "Foo")
        );
    }
}
