use std::fs;
use std::path::Path;

pub fn run(args: &[String]) {
    if args.is_empty() {
        eprintln!("mkdir: missing operand");
        return;
    }

    let mut create_parents = false;
    let mut operands: Vec<&str> = Vec::new();
    let mut end_of_opts = false;

    for a in args {
        if end_of_opts {
            operands.push(a.as_str());
            continue;
        }

        if a == "--" {
            end_of_opts = true;
        } else if a == "-p" {
            create_parents = true;
        } else if a.starts_with('-') && a != "-" {
            // Unknown option (we only support -p)
            eprintln!("mkdir: invalid option '{}'", a);
        } else {
            operands.push(a.as_str());
        }
    }

    if operands.is_empty() {
        eprintln!("mkdir: missing operand");
        return;
    }

    for op in operands {
        let p = Path::new(op);
        let res = if create_parents { fs::create_dir_all(p) } else { fs::create_dir(p) };

        if let Err(e) = res {
            eprintln!("mkdir: cannot create directory '{}': {}", op, e);
        }
    }
}
