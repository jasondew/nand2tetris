use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod translator;

fn read_file(path: &String) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    if let Some(path) = env::args().nth(1) {
        if let Ok(contents) = read_file(&path) {
            let name: &str =
                Path::new(&path).file_stem().unwrap().to_str().unwrap();
            contents
                .lines()
                .map(|line| line.trim())
                .filter(|line| !(line.starts_with("//") || line.is_empty()))
                .for_each(|line| {
                    for instruction in translator::translate(line, name) {
                        println!("{}", instruction);
                    }
                    println!()
                });
            println!("// infinite loop\n@INFINITY\n(INFINITY)\n0;JMP");
        } else {
            println!("ERROR: unable to read file");
        }
    } else {
        println!("USAGE: ./stack-to-hack file.vm");
    }
}
