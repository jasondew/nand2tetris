use jack_compiler::compiler;
use std::env;
use std::fs;
use std::path::Path;

fn compile<S>(path: S)
where
    S: AsRef<Path> + std::fmt::Display,
{
    if let Ok(contents) = fs::read_to_string(&path) {
        for instruction in compiler::compile(contents) {
            println!("{instruction}");
        }
    } else {
        panic!("ERROR: unable to read {path}");
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
                        panic!("{error:?}")
                    }
                }
            }
        }
        Err(error) => {
            panic!("{error:?}")
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
        println!("USAGE: ./compile <*.jack or directory>");
    }
}
