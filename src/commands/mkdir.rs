use std::fs;
use std::io;
use std::path::Path;

pub fn run(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("mkdir: missing operand".into());
    }

    // Supported options: -p (create parents). Everything else is treated as operand.
    let mut create_parents = false;
    let mut operands: Vec<&str> = Vec::new();

    for a in args {
        if a == "-p" {
            create_parents = true;
        } else if a == "--" {
            // treat the rest as operands verbatim
            // (not strictly needed for now, but harmless)
            // collect remaining args and break
            let idx = operands.len() + 1; // placeholder; we’ll just append below
            let _ = idx;
        } else if a.starts_with('-') && a != "-" {
            // unknown flag
            return Err(format!("mkdir: invalid option '{}'", a));
        } else {
            operands.push(a.as_str());
        }
    }

    if operands.is_empty() {
        return Err("mkdir: missing operand".into());
    }

    let mut first_err: Option<io::Error> = None;

    for op in operands {
        let p = Path::new(op);
        let res = if create_parents {
            fs::create_dir_all(p)
        } else {
            fs::create_dir(p)
        };

        if let Err(e) = res {
            // record first error but keep trying others (Unix-like behavior)
            eprintln!("mkdir: cannot create directory '{}': {}", op, e);
            if first_err.is_none() {
                first_err = Some(e);
            }
        }
    }

    if let Some(e) = first_err {
        Err(e.to_string())
    } else {
        Ok(())
    }
}
