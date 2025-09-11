use std::fs::File;
use std::io::{ self, Write, Read };

pub fn run(files: &[String]) {
    if files.is_empty() || files == &["-"] {
        let mut buffer = String::new();
        io::stdout().flush().unwrap();

        loop {
            match io::stdin().read_line(&mut buffer) {
                Ok(0) => {
                    break;
                }
                Ok(_) => {
                    print!("{}", buffer);
                    buffer.clear();
                }
                Err(e) => {
                    eprintln!("Error reading from stdin: {}", e);
                    break;
                }
            }
        }
    } else {
        for file in files {
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
}
