use jack_compiler::compiler::tokenizer;
use std::env;
use std::fs;
use std::io::prelude::*;

fn tokenize<S>(path: S)
where
    S: AsRef<str>,
{
    if let Ok(contents) = read_file(&path) {
        println!("<tokens>");
        for token in tokenizer::tokenize(contents) {
            use tokenizer::Token::*;
            let (name, value) = match token {
                Keyword(string) => ("keyword", string),
                Symbol(string) => ("symbol", escape(string)),
                IntegerConstant(number) => {
                    ("integerConstant", number.to_string())
                }
                StringConstant(string) => ("stringConstant", string),
                Identifier(string) => ("identifier", string),
            };
            println!("<{}> {} </{}>", name, value, name);
        }
        println!("</tokens>");
    } else {
        panic!("ERROR: unable to read file {}", path.as_ref());
    }
}

fn escape(string: String) -> String {
    match string.as_ref() {
        "<" => "&lt;".into(),
        ">" => "&gt;".into(),
        "&" => "&amp;".into(),
        "\"" => "&quot;".into(),
        _ => string,
    }
}

fn read_file<S>(path: S) -> std::io::Result<String>
where
    S: AsRef<str>,
{
    let mut file = fs::File::open(path.as_ref())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    if let Some(path) = env::args().nth(1) {
        tokenize(path);
    } else {
        println!("USAGE: ./tokenize Foo.jack");
    }
}
