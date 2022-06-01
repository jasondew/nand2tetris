use jack_compiler::compiler::debug;
use jack_compiler::compiler::tokenizer;
use std::env;
use std::fs;
use std::path::Path;

fn tokenize<S>(path: S)
where
    S: AsRef<Path> + std::fmt::Display,
{
    if let Ok(contents) = fs::read_to_string(&path) {
        debug::print_tokens(tokenizer::tokenize(contents));
    } else {
        panic!("ERROR: unable to read file {path}");
    }
}

fn main() {
    if let Some(path) = env::args().nth(1) {
        tokenize(path);
    } else {
        println!("USAGE: ./tokenize Foo.jack");
    }
}
