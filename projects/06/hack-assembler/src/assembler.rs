mod code;
mod instruction;
mod parser;

pub fn compile(contents: &String) -> Vec<String> {
    contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| !(line.starts_with("//") || line.is_empty()))
        .map(|line| parser::parse(&line))
        .map(|line| code::encode(&line))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ignores_comments() {
        assert_eq!(
            Vec::<String>::new(),
            compile(&"// this is a comment\n// and another one".to_string())
        );
    }

    #[test]
    fn test_ignores_blank_lines() {
        assert_eq!(Vec::<String>::new(), compile(&"  \n\n".to_string()));
    }

    #[test]
    fn test_a_instruction() {
        assert_eq!(vec!["0000000000000000"], compile(&"@0".to_string()));
        assert_eq!(vec!["0000000000000111"], compile(&"@7".to_string()));
        assert_eq!(vec!["0111111111111111"], compile(&"@32767".to_string()));
    }

    #[test]
    fn test_c_instruction() {
        assert_eq!(vec!["1111110000010000"], compile(&"D=M".to_string()));
        assert_eq!(vec!["1110000010010000"], compile(&"D=D+A".to_string()));
        assert_eq!(
            vec!["1111010101111010"],
            compile(&"AMD=D|M;JEQ".to_string())
        );
    }
}
