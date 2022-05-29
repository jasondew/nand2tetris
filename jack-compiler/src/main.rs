use std::env;
use std::fs;
use std::io::prelude::*;
// use std::path::Path;

fn compile<S>(path: S)
where
    S: AsRef<str>,
{
    if let Ok(contents) = read_file(&path) {
        dbg!(compile(contents));
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
                            panic!("{:?}", error);
                        }
                    }
                }
            }
            Err(_) => {
                compile(path);
            }
        }
    } else {
        println!("USAGE: ./jack-compiler Foo.jack");
    }
}
