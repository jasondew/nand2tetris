use jack_compiler::compiler::debug;
use jack_compiler::compiler::parser;
use std::env;
use std::fs;
use std::io::prelude::*;

fn parse<S>(path: S)
where
    S: AsRef<str>,
{
    if let Ok(contents) = read_file(&path) {
        let class = parser::parse(contents);
        debug::print_class(class)
    } else {
        panic!("ERROR: unable to read file {}", path.as_ref());
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
        parse(path);
    } else {
        println!("USAGE: ./parse Foo.jack");
    }
}
