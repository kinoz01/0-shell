mod shell;
mod commands;

use std::io;

fn main() -> io::Result<()> {
    print!("\x1b[2J\x1b[H");

    loop {
        let Some(cmdline) = shell::read_command()? else {
            println!();
            break;
        };
        if cmdline.trim().is_empty() {
            continue;
        }
        if shell::dispatch(cmdline.trim_end()) {
            break;
        }
    }

    Ok(())
}
