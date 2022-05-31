use jack_compiler::compiler;
use std::env;
use std::fs;
use std::path::Path;

fn compile<S>(path: S)
where
    S: AsRef<Path> + std::fmt::Display,
{
    match fs::read_to_string(&path) {
        Ok(contents) => {
            dbg!(compiler::compile(contents));
        }
        Err(error) => {
            panic!("ERROR: unable to read file {}: {}", &path, error);
        }
    }
}

fn compile_directory<S>(path: S)
where
    S: AsRef<Path> + std::fmt::Display,
{
    match fs::read_dir(&path) {
        Ok(read_dir) => {
            for maybe_entry in read_dir {
                match maybe_entry {
                    Ok(entry) => {
                        if entry.path().extension().unwrap() == "jack" {
                            compile(entry.path().to_str().unwrap());
                        }
                    }
                    Err(error) => {
                        panic!("{:?}", error)
                    }
                }
            }
        }
        Err(error) => {
            panic!("{:?}", error)
        }
    }
}

fn main() {
    if let Some(path) = env::args().nth(1) {
        if fs::metadata(&path).unwrap().is_dir() {
            compile_directory(path);
        } else {
            compile(path);
        }
    } else {
        println!("USAGE: ./jack-compiler Foo.jack");
    }
}
