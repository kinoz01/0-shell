use std::fs;
use std::path::Path;

pub fn run(args: &[String]) {
    if args.is_empty() {
        eprintln!("mkdir: missing operand");
    }

    let mut create_parents = false;
    let mut operands: Vec<&str> = Vec::new();

    for a in args {
        if a == "-p" {
            create_parents = true;
        } else if a == "--" {
            // for now: treat the rest as operands; adjust if you later implement it fully
            // (you can break here once you handle it)
        } else if a.starts_with('-') && a != "-" {
            eprintln!("mkdir: invalid option '{}'", a);
        } else {
            operands.push(a.as_str());
        }
    }

    if operands.is_empty() {
        eprintln!("mkdir: missing operand");
    }

    for op in operands {
        let p = Path::new(op);
        let res = if create_parents {
            fs::create_dir_all(p)
        } else {
            fs::create_dir(p)
        };

        if let Err(e) = res {
            eprintln!("mkdir: cannot create directory '{}': {}", op, e);
        }
    }

}
