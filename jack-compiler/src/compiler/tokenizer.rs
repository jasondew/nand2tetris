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

    Token::IntegerConstant(u16::from_str_radix(&digits, 10).unwrap())
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
            panic!("unexpected '{}' while parsing multi-line comment", other)
        }
        None => panic!("unexpected EoF while parsing multi-line comment"),
    }
}

fn parse_single_line_comment(chars: &mut Peekable<Chars>) {
    match chars.next() {
        Some('*') => discard_until(chars, '/'),
        Some('/') => discard_until(chars, '\n'),
        Some(other) => panic!("unexpected '{}' while parsing comment", other),
        None => panic!("unexpected EoF while parsing comment"),
    }
}

fn read_until_non_identifier(
    first_char: char,
    chars: &mut Peekable<Chars>,
) -> String {
    let mut read_chars = vec![first_char];

    while let Some(next_char) = chars.peek() {
        if next_char.is_ascii_alphanumeric() || *next_char == '_' {
            read_chars.push(chars.next().unwrap());
        } else {
            break;
        }
    }

    read_chars.into_iter().collect()
}

fn read_until_non_numeric(
    first_char: char,
    chars: &mut Peekable<Chars>,
) -> String {
    let mut read_chars = vec![first_char];

    while let Some(next_char) = chars.peek() {
        if next_char.is_numeric() {
            read_chars.push(chars.next().unwrap());
        } else {
            break;
        }
    }

    read_chars.into_iter().collect()
}

fn read_until(chars: &mut Peekable<Chars>, stop_char: char) -> String {
    let mut string_chars = vec![];

    while let Some(next_char) = chars.peek() {
        if *next_char == stop_char {
            break;
        } else {
            string_chars.push(chars.next().unwrap());
        }
    }

    string_chars.into_iter().collect::<String>()
}

fn discard_until(chars: &mut Peekable<Chars>, final_char: char) {
    match chars.next() {
        Some(next_char) if next_char == final_char => {}
        Some(_) => discard_until(chars, final_char),
        None => panic!("unexpected EoF while looking for '{}'", final_char),
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
            "unexpected EoF while looking for '{}{}'",
            penultimate_char, final_char
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
