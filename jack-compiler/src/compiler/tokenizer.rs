use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(String),
    Symbol(String),
    IntegerConstant(u16),
    StringConstant(String),
    Identifier(String),
}

impl AsRef<Token> for Token {
    fn as_ref(&self) -> &Token {
        self
    }
}

pub fn tokenize<S>(input: S) -> Vec<Token>
where
    S: AsRef<str>,
{
    use Token::*;

    let mut tokens = Vec::new();
    let mut chars: Peekable<Chars> = input.as_ref().chars().peekable();

    while let Some(next_char) = chars.next() {
        if !next_char.is_whitespace() {
            match next_char {
                '/' if chars.peek() == Some(&'*') => {
                    parse_multiline_comment(&mut chars)
                }
                '/' if chars.peek() == Some(&'/') => {
                    parse_single_line_comment(&mut chars)
                }
                '{' | '}' | '(' | ')' | '[' | ']' | '.' | ',' | ';' | '+'
                | '-' | '*' | '/' | '&' | '|' | '<' | '>' | '=' | '~' => {
                    tokens.push(Symbol(next_char.to_string()));
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    tokens.push(parse_integer_constant(next_char, &mut chars));
                }
                '"' => {
                    tokens.push(parse_string_constant(&mut chars));
                }
                _ => {
                    tokens.push(parse_identifier_or_keyword(
                        next_char, &mut chars,
                    ));
                }
            }
        }
    }

    tokens
}

fn parse_identifier_or_keyword(
    first_char: char,
    chars: &mut Peekable<Chars>,
) -> Token {
    let string: String = read_until_non_identifier(first_char, chars);

    match string.as_ref() {
        "class" | "constructor" | "function" | "method" | "field"
        | "static" | "var" | "int" | "char" | "boolean" | "void" | "true"
        | "false" | "null" | "this" | "let" | "do" | "if" | "else"
        | "while" | "return" => Token::Keyword(string),
        _ => Token::Identifier(string),
    }
}

fn parse_integer_constant(
    first_digit: char,
    chars: &mut Peekable<Chars>,
) -> Token {
    let digits: String = read_until_non_numeric(first_digit, chars);

    Token::IntegerConstant(digits.parse::<u16>().unwrap())
}

fn parse_string_constant(chars: &mut Peekable<Chars>) -> Token {
    let string = read_until(chars, '"');
    discard_until(chars, '"');
    Token::StringConstant(string)
}

fn parse_multiline_comment(chars: &mut Peekable<Chars>) {
    match chars.next() {
        Some('*') => {
            discard_until_with_peek(chars, '*', '/');
        }
        Some(other) => {
            panic!("unexpected '{other}' while parsing multi-line comment")
        }
        None => panic!("unexpected EoF while parsing multi-line comment"),
    }
}

fn parse_single_line_comment(chars: &mut Peekable<Chars>) {
    match chars.next() {
        Some('*') => discard_until(chars, '/'),
        Some('/') => discard_until(chars, '\n'),
        Some(other) => panic!("unexpected '{other}' while parsing comment"),
        None => panic!("unexpected EoF while parsing comment"),
    }
}

fn read_until_non_identifier(
    first_char: char,
    chars: &mut Peekable<Chars>,
) -> String {
    let mut identifier = format!("{first_char}");

    while let Some(next_char) =
        chars.next_if(|&c| c.is_ascii_alphanumeric() || c == '_')
    {
        identifier.push(next_char);
    }

    identifier
}

fn read_until_non_numeric(
    first_digit: char,
    chars: &mut Peekable<Chars>,
) -> String {
    let mut digits = format!("{first_digit}");

    while let Some(next_digit) = chars.next_if(|&c| c.is_numeric()) {
        digits.push(next_digit);
    }

    digits
}

fn read_until(chars: &mut Peekable<Chars>, stop_char: char) -> String {
    let mut string = String::new();

    while let Some(next_char) = chars.next_if(|&c| c != stop_char) {
        string.push(next_char);
    }

    string
}

fn discard_until(chars: &mut Peekable<Chars>, stop_char: char) {
    while let Some(next_char) = chars.next() {
        if next_char == stop_char {
            return;
        }
    }
}

fn discard_until_with_peek(
    chars: &mut Peekable<Chars>,
    penultimate_char: char,
    final_char: char,
) {
    match chars.next() {
        Some(next_char)
            if next_char == penultimate_char
                && chars.peek() == Some(&final_char) =>
        {
            chars.next();
        }
        Some(_) => discard_until_with_peek(chars, penultimate_char, final_char),
        None => panic!(
            "unexpected EoF while looking for '{penultimate_char}{final_char}'"
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::Token::*;
    use super::*;

    #[test]
    fn test_ignores_comments() {
        assert_eq!(tokenize("/* multi-line\r\ncomment */"), vec![]);
        assert_eq!(tokenize("/** API-style comment */"), vec![]);
        assert_eq!(tokenize("// single-line comment\n"), vec![]);
    }

    #[test]
    fn test_tokenizer() {
        assert_eq!(
            tokenize(
                "\
            if (x < 42) {
                // handles the sign
                let sign_86 = \"negative\";
            }
        "
            ),
            vec![
                Keyword("if".into()),
                Symbol("(".into()),
                Identifier("x".into()),
                Symbol("<".into()),
                IntegerConstant(42),
                Symbol(")".into()),
                Symbol("{".into()),
                Keyword("let".into()),
                Identifier("sign_86".into()),
                Symbol("=".into()),
                StringConstant("negative".into()),
                Symbol(";".into()),
                Symbol("}".into()),
            ]
        );
    }
}
