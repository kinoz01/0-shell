mod shell;
mod commands;

use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut line = String::new();
    print!("\x1b[2J\x1b[H");

    loop {
        // prompt
        print!("{}", shell::prompt());
        io::stdout().flush()?;

        line.clear();
        let n = io::stdin().read_line(&mut line)?;
        if n == 0 {
            println!(); // Ctrl+D
            break;
        }

        let cmdline = line.trim_end();
        if cmdline.is_empty() {
            continue;
        }


        match shell::dispatch(cmdline) {
            true => break,
            _ => {}
        }
    }

    Ok(())
}
