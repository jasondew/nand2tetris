use std::env;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

mod translator;

fn translate<S>(path: S)
where
    S: AsRef<str>,
{
    if let Ok(contents) = read_file(&path) {
        let name: &str = Path::new(path.as_ref())
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        contents.lines().map(|line| line.trim()).for_each(|line| {
            if line.is_empty() || line.starts_with("//") {
                println!("{}", line);
            } else {
                for instruction in translator::translate(line, name) {
                    println!("{}", instruction);
                }
                println!();
            }
        });
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
        for instruction in translator::bootstrap() {
            println!("{}", instruction);
        }
        println!();

        match fs::read_dir(&path) {
            Ok(read_dir) => {
                for maybe_entry in read_dir {
                    match maybe_entry {
                        Ok(entry) => {
                            if entry.path().extension().unwrap() == "vm" {
                                translate(entry.path().to_str().unwrap());
                            }
                        }
                        Err(error) => {
                            panic!("{:?}", error);
                        }
                    }
                }
            }
            Err(_) => {
                translate(path);
            }
        }
    } else {
        println!("USAGE: ./stack-to-hack file.vm");
    }
}
