use std::fs::*;
use std::path::*;

pub fn run(args: &[String]){
    if args.len() >= 1 {
        if args[0].starts_with("-") && args[0].len() != 1 {
            if args.len() == 1 && args[0] == "-r" {
                println!("rm: missing operand");
                return;
            }
            if args[0] != "-r"{
                println!("rm: invalid option -- '{}'", args[0].strip_prefix("-").unwrap());
                return;
            }
            for arg in args[1..].iter() {
                let path = Path::new(arg);
                if path.exists() {
                    if path.is_dir() {
                        remove_dir_all(path);
                    }else if path.is_file(){
                        remove_file(path);
                    }
                } else {
                    println!("rm: cannot remove '{}': No such file or directory", arg);
                }
            }
        } else {
            for arg in args.iter() {
                let path = Path::new(arg);
                if path.exists() {
                    if path.is_dir() {
                        println!("rm: cannot remove '{}': Is a directory", arg);
                    } else if path.is_file() {
                        remove_file(path);
                    }
                }else {
                    println!("rm: cannot remove '{}': No such file or directory", arg);
                }
            }
        }
    }

}
