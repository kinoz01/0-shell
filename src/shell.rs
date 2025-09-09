use crate::commands::*;
use std::io::{ self, Write };

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Mode {
    Normal,
    InSingle,
    InDouble,
}

pub fn dispatch(input: &str) -> bool {
    let cmd_args = parse_input(input);
    if cmd_args.is_empty() {
        return false;
    }

    let cmd = &cmd_args[0];
    let args = &cmd_args[1..];

    match cmd.as_str() {
        "mkdir" => mkdir::run(args),
        "ls" => ls::run(args),
        "cd" => cd::run(args),
        "cat"  => cat::run(args),
        "pwd"  => pwd::run(),
        "cp"   => cp::run(args),
        //"rm"   => rm::run(args),
        //"mv"   => mv::run(args),
        "echo" => echo::run(args.to_vec()),
        "clear" => clear::run(),
        "exit" => return true,
        other => eprintln!("Command '{}' not found", other),
    }

    false
}

fn parse_input(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut chars = s.chars().peekable();
    let mut mode = Mode::Normal;

    // started: have we begun the current word?
    // first_qe: the very first char of the word was quoted or escaped
    let mut started = false;
    let mut first_qe = false;

    let push_word = |out: &mut Vec<String>, cur: &mut String, started: &mut bool, first_qe: &mut bool| {
        if !cur.is_empty() {
            if !*first_qe && cur.starts_with('~') {
                expand_tilde(cur);
            }
            out.push(std::mem::take(cur));
        }
        *started = false;
        *first_qe = false;
    };

    while let Some(c) = chars.next() {
        match (mode, c) {
            // -------- Normal mode --------
            (Mode::Normal, '\\') => {
                match chars.peek().copied() {
                    Some(next) if matches!(next, ' ' | '"' | '\\' | '\'' | '~') => {
                        if !started { started = true; first_qe = true; }
                        cur.push(next);
                        chars.next();
                    }
                    Some(_) => {
                        if !started { started = true; }
                        cur.push('\\');
                    }
                    None => {
                        if !started { started = true; }
                        cur.push('\\');
                    }
                }
            }
            (Mode::Normal, '"') => {
                // entering "
                if !started { started = true; first_qe = true; }
                mode = Mode::InDouble;
            }
            (Mode::Normal, '\'') => {
                // entering '
                if !started { started = true; first_qe = true; }
                mode = Mode::InSingle;
            }
            (Mode::Normal, ch) if ch.is_whitespace() => {
                push_word(&mut out, &mut cur, &mut started, &mut first_qe);
            }
            (Mode::Normal, other) => {
                if !started { started = true;}
                cur.push(other);
            }

            // -------- Inside "double quotes" --------
            (Mode::InDouble, '\\') => {
                match chars.peek().copied() {
                    Some(next) if matches!(next, '"' | '\\' | ' ') => {
                        // escape limited set in "
                        cur.push(next);
                        chars.next();
                    }
                    Some(_) | None => {
                        cur.push('\\');
                    }
                }
            }
            (Mode::InDouble, '"') => {
                mode = Mode::Normal;
            }
            (Mode::InDouble, other) => {
                cur.push(other);
            }

            // -------- Inside 'single quotes' --------
            (Mode::InSingle, '\'') => {
                mode = Mode::Normal;
            }
            (Mode::InSingle, other) => {
                cur.push(other);
            }
        }
    }

    if !cur.is_empty() {
        if !first_qe && cur.starts_with('~') {
            expand_tilde(&mut cur);
        }
        out.push(cur);
    }

    out
}

fn expand_tilde(word: &mut String) {
    if let Ok(home) = std::env::var("HOME") {
        if word == "~" {
            *word = home;
        } else if let Some(rest) = word.strip_prefix("~/") {
            *word = format!("{home}/{rest}");
        }
    }
}

// Read a full command, possibly spanning multiple lines if quotes are left open.
pub fn read_command() -> io::Result<Option<String>> {
    let mut buf = String::new();
    let mut filled = false;

    // ----- primary prompt -----
    print!("{}", prompt());
    io::stdout().flush()?;
    let mut line = String::new();
    let n = io::stdin().read_line(&mut line)?;
    if n == 0 {
        return Ok(None); // EOF as line (ctr+d)
    }

    buf.push_str(&line);

    // ----- while quotes remain open, keep reading continuation lines -----
    loop {
        match quote_status(&buf) {
            Mode::Normal => {
                return Ok(Some(buf));
            }
            Mode::InSingle | Mode::InDouble => {
                if line.ends_with('\n') {
                    print!(" > ");
                    io::stdout().flush()?;
                    filled = false;
                }
            }
        }

        line.clear();
        let n = io::stdin().read_line(&mut line)?;
        if n != 0 && !filled {
            filled = true;
        }

        if n == 0 && !filled {
            eprintln!("\nUnexpected EOF");
            return Ok(Some(String::new()));
        }
        
        buf.push_str(&line); 
    }
}

pub fn prompt() -> String {
    const GREEN: &str = "\x1b[32m";
    const BLUE: &str = "\x1b[34m";
    const RESET: &str = "\x1b[0m";

    let user = std::env
        ::var("USER")
        .or_else(|_| std::env::var("LOGNAME"))
        .unwrap_or_else(|_| "user".to_string());

    let host = hostname();

    let cwd_full = std::env
        ::current_dir()
        .ok()
        .and_then(|p| p.into_os_string().into_string().ok())
        .unwrap_or_else(|| "?".to_string());

    let home = std::env::var("HOME").unwrap_or_default();
    let cwd_disp = if !home.is_empty() && cwd_full.starts_with(&home) {
        format!("~{}", &cwd_full[home.len()..])
    } else {
        cwd_full
    };

    format!("{GREEN}{user}@{host}{RESET}:{BLUE}{cwd_disp}{RESET}$ ")
}

fn hostname() -> String {
    if let Ok(h) = std::env::var("HOSTNAME") {
        return h;
    }
    if let Ok(s) = std::fs::read_to_string("/proc/sys/kernel/hostname") {
        return s.trim().to_string();
    }
    if let Ok(s) = std::fs::read_to_string("/etc/hostname") {
        return s.trim().to_string();
    }
    "host".to_string()
}

// Lightweight scanner to check if the input ends with unmatched quotes.
// Returns the final mode. If it's not 'Mode::Normal', you're still inside quotes.
pub fn quote_status(s: &str) -> Mode {
    let mut mode = Mode::Normal;
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        match (mode, c) {
            // normal
            (Mode::Normal, '\\') => {
                if let Some(&next) = chars.peek() {
                    if next == '"' || next == '\'' {
                        chars.next();
                    }
                }
            }
            (Mode::Normal, '"') => {
                mode = Mode::InDouble;
            }
            (Mode::Normal, '\'') => {
                mode = Mode::InSingle;
            }
            (Mode::Normal, _) => {}

            // inside double quote
            (Mode::InDouble, '\\') => {
                if let Some(&next) = chars.peek() {
                    // Inside "": escape " \ and space
                    if next == '"' || next == '\\' || next == ' ' {
                        chars.next();
                    }
                }
            }
            (Mode::InDouble, '"') => {
                mode = Mode::Normal;
            }
            (Mode::InDouble, _) => {}

            // inside single quote
            (Mode::InSingle, '\'') => {
                mode = Mode::Normal;
            }
            (Mode::InSingle, _) => {}
        }
    }

    mode
}
