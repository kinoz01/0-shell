use std::io::{self, Write};

pub fn run() {
    print!("\x1b[2J\x1b[H");
    let _ = io::stdout().flush();
}