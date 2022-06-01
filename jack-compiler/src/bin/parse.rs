use jack_compiler::compiler::debug;
use jack_compiler::compiler::parser;
use std::env;
use std::fs;
use std::path::Path;

fn parse<S>(path: S)
where
    S: AsRef<Path> + std::fmt::Display,
{
    if let Ok(contents) = fs::read_to_string(&path) {
        debug::print_class(parser::parse(contents));
    } else {
        panic!("ERROR: unable to read file {path}");
    }
}

fn main() {
    if let Some(path) = env::args().nth(1) {
        parse(path);
    } else {
        println!("USAGE: ./parse Foo.jack");
    }
}
