use crate::commands;
use std::fmt;

#[derive(Debug)]
pub enum ShellError {
    Message(String),
}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShellError::Message(m) => write!(f, "{m}"),
        }
    }
}

pub fn dispatch(input: &str) -> Result<bool, ShellError> {
    let tokens = tokenize(input);
    if tokens.is_empty() {
        return Ok(true);
    }

    let cmd = &tokens[0];
    let args = &tokens[1..];

    match cmd.as_str() {
        // only mkdir implemented for now
        "mkdir" => {
            commands::mkdir::run(args).map_err(ShellError::Message)?;
            Ok(true)
        }

        // keep 'exit' placeholder for later; user can Ctrl+D meanwhile
        "exit" => Ok(false),

        other => {
            eprintln!("Command '{other}' not found");
            Ok(true)
        }
    }
}

// Minimal tokenizer:
// - splits on ASCII whitespace
// - supports double-quoted segments "like this"
// - supports backslash escapes for space and quotes: \  \"
fn tokenize(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut chars = s.chars().peekable();
    let mut in_quotes = false;

    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if let Some(&next) = chars.peek() {
                    // allow escaping space and quote and backslash
                    if next == ' ' || next == '"' || next == '\\' {
                        cur.push(next);
                        chars.next();
                    } else {
                        cur.push(c);
                    }
                } else {
                    cur.push(c);
                }
            }
            '"' => {
                in_quotes = !in_quotes;
            }
            c if c.is_whitespace() && !in_quotes => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
            }
            _ => cur.push(c),
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

pub fn prompt() -> String {
    // ANSI colors
    const GREEN: &str = "\x1b[32m";
    const BLUE: &str = "\x1b[34m";
    const RESET: &str = "\x1b[0m";

    let user = std::env::var("USER")
        .or_else(|_| std::env::var("LOGNAME"))
        .unwrap_or("user".to_string());
    let host = hostname();

    let cwd_full = std::env::current_dir()
        .ok()
        .and_then(|p| p.into_os_string().into_string().ok())
        .unwrap_or_else(|| "?".to_string());

    let home = std::env::var("HOME").unwrap_or_default();
    let cwd_disp = if !home.is_empty() && cwd_full.starts_with(&home) {
        format!("~{}", &cwd_full[home.len()..])
    } else {
        cwd_full
    };

    // kino@pop-os:~/Videos/0-shell$
    format!("{GREEN}{user}@{host}{RESET}:{BLUE}{cwd_disp}{RESET}$ ")
}

fn hostname() -> String {
    // Try env, then /proc, then /etc
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
