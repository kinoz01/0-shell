use crate::commands::*;

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
        "cd"  => cd::run(args),
        "cat"  => cat::run(args),
        "pwd" => pwd::run(),
        //"cp" => cp::run(args),
        //"rm" => rm::run(args),
        //"mv" => mv::run(args),
        "echo" => echo::run(args.to_vec()),
        "exit" => return true,
        other => eprintln!("Command '{}' not found", other),
    }

    return false;
}

fn parse_input(s: &str) -> Vec<String> {
    #[derive(Copy, Clone, PartialEq)]
    enum Mode {
        Normal,
        InSingle,
        InDouble,
    }

    let mut out = Vec::new();
    let mut cur = String::new();
    let mut chars = s.chars().peekable();
    let mut mode = Mode::Normal;

    while let Some(c) = chars.next() {
        match (mode, c) {
            // -------- Normal mode --------
            (Mode::Normal, '\\') => {
                if let Some(&next) = chars.peek() {
                    // allow escaping space, quotes, and backslash
                    if next == ' ' || next == '"' || next == '\\' || next == '\'' {
                        cur.push(next);
                        chars.next();
                    } else {
                        cur.push('\\');
                    }
                } else {
                    cur.push('\\');
                }
            }
            (Mode::Normal, '"') => {
                mode = Mode::InDouble;
            }
            (Mode::Normal, '\'') => {
                mode = Mode::InSingle;
            }
            (Mode::Normal, c) if c.is_whitespace() => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
            }
            (Mode::Normal, other) => cur.push(other),

            // -------- Inside "double quotes" --------
            (Mode::InDouble, '\\') => {
                if let Some(&next) = chars.peek() {
                    // escape " \ and space inside "
                    if next == '"' || next == '\\' || next == ' ' {
                        cur.push(next);
                        chars.next();
                    } else {
                        cur.push('\\');
                    }
                } else {
                    cur.push('\\');
                }
            }
            (Mode::InDouble, '"') => {
                mode = Mode::Normal;
            }
            (Mode::InDouble, other) => cur.push(other),

            // -------- Inside 'single quotes' --------
            (Mode::InSingle, '\'') => {
                mode = Mode::Normal;
            }
            (Mode::InSingle, other) => cur.push(other),
        }
    }

    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

pub fn prompt() -> String {
    const GREEN: &str = "\x1b[32m";
    const BLUE: &str = "\x1b[34m";
    const RESET: &str = "\x1b[0m";

    let user = std::env::var("USER")
        .or_else(|_| std::env::var("LOGNAME"))
        .unwrap_or_else(|_| "user".to_string());

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
