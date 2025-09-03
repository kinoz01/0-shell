use std::env;
use std::path::Path;

pub fn run(args: &[String]) {
    // Decide the target directory
    let target = match args.len() {
        0 =>
            match env::var("HOME") {
                Ok(h) => h,
                Err(_) => {
                    eprintln!("cd: HOME not set");
                    return;
                }
            }
        1 => {
            let a = &args[0];
            if a == "-" {
                match env::var("OLDPWD") {
                    Ok(old) => {
                        println!("{old}");
                        old
                    }
                    Err(_) => {
                        eprintln!("cd: OLDPWD not set");
                        return;
                    }
                }
            } else {
                a.to_string()
            }
        }
        _ => {
            eprintln!("cd: too many arguments");
            return;
        }
    };

    // Remember current dir (for OLDPWD)
    let old = match env::current_dir() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("cd: failed to get current dir: {e}");
            return;
        }
    };

    // Try to change directory
    if let Err(e) = env::set_current_dir(Path::new(&target)) {
        eprintln!("cd: {}: {}", target, e);
        return;
    }

    // Update PWD/OLDPWD
    if let Ok(new) = env::current_dir() {
        env::set_var("OLDPWD", old);
        env::set_var("PWD", &new);
    }
}
