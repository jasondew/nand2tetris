pub mod parser;
pub mod tokenizer;
pub mod debug;

pub fn compile<S>(input: S) -> parser::Class
where
    S: AsRef<str>,
{
    parser::parse(input)
}
