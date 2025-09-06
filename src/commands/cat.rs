use std::env;
use std::fs::File;
use std::io::Read;

pub fn run(files: &[String]) {
    for file in files {
        let mut path = file;
        match File::open(file) {
            Ok(mut f) => {
                let mut buffer: String = String::new();
                match f.read_to_string(&mut buffer) {
                    Ok(_) => print!("{}", buffer),
                    Err(e) => eprintln!("Failed to read {}: {}", file, e),
                }
            }
            Err(e) => {
                eprintln!("Failed to open {}: {}", file, e);
            }
        }
    }
}
