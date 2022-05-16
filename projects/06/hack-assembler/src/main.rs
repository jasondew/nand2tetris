use std::env;
use std::fs::File;
use std::io::prelude::*;

mod assembler;

fn read_file(path: &String) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    if let Some(path) = env::args().skip(1).next() {
        if let Ok(contents) = read_file(&path) {
            for line in assembler::compile(&contents) {
                println!("{}", line);
            }
        } else {
            println!("ERROR: unable to read file");
        }
    } else {
        println!("USAGE: ./hack-assembler file.asm");
    }
}
