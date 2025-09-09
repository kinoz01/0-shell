use std::path::*;
use std::fs::*;


fn copy_dir(src: &Path, dst: &Path) -> std::io::Result<()>{
   if src.is_dir() {
       create_dir(dst.join(src));
        for e in read_dir(src)? {
            let e = e?;
            copy_dir(&e.path(), dst);
        }
   }else {
       if let Some(name) = src.file_name() {
            copy(src, dst.join(Path::new(name)));
       }
   }
   Ok(())
}

pub fn run(args: Vec<String>) {
     if args.len() > 1 {
        let mut paths = Vec::new();
        let mut nonepaths = Vec::new();
        for (i, arg) in args.iter().enumerate() {
            if metadata(arg).is_ok() {
                paths.push((arg, i));
            }else {
                 if i == args.len() - 1 && args.len() != 2 {
                    println!("mv: target '{}' is not a directory", arg);
                    return;
                } else {
                    nonepaths.push((arg, i));
                }
            }
        }
       
        if paths.len() == 1 && nonepaths.len() == 1 {
                if paths[0].1 < nonepaths[0].1 {
                    rename(Path::new(paths[0].0), Path::new(nonepaths[0].0));
                }else {
                    println!("mv: cannot stat '{}': No such file or directory", nonepaths[0].0);
                }
            return;
        } else if paths.len() == 2 && nonepaths.len() == 0 {
                let src = Path::new(paths[0].0);
                let dst = Path::new(paths[1].0);
                if src.is_dir() && dst.is_file() {
                    println!("mv: cannot overwrite non-directory '{}' with directory '{}'", paths[1].0, paths[0].0);
                }else {
                    copy_dir(src, dst);
                    if src.is_dir() {
                        remove_dir_all(src);
                    } else {
                        remove_file(src);
                    }
                }
                return
        }
        let target = Path::new(&paths[paths.len()-1].0);
        if !target.exists() {
            println!("mv: target '{:?}' is not a directory", target.file_name());
            return;
        }
        for (i, p) in paths.iter().enumerate() {
            if i != paths.len() - 1{
                copy_dir(Path::new(p.0), target);
            }
        }
        for (i, p) in paths.iter().enumerate() {
            if i != paths.len() - 1{
                let path = Path::new(p.0);
                if path.is_dir() {
                    remove_dir_all(path);
                } else {
                    remove_file(path);
                }
            }
        }
        nonepaths.iter().for_each(|c| println!("mv: cannot stat '{}': No such file or directory", c.0));
    }else if args.len() == 1 {
        println!("mv: missing destination file operand after '{}'", args[0]);
    }else {
        println!("mv: missing file operand");
    }
}
