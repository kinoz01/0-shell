use std::env;

pub fn run() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => {
            eprintln!("Error getting current directory: {}", e);
        }
    }
}
