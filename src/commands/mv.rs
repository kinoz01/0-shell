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
                paths.push(arg);
            }else {
                 if i == args.len() - 1 && args.len() != 2 {
                    println!("mv: target '{}' is not a directory", arg);
                    return;
                } else {
                    nonepaths.push(arg);
                }
            }
        }
        if paths.len() == 1 && nonepaths.len() == 1 {
            rename(Path::new(paths[0]), Path::new(nonepaths[0]));
            return;
        }
        let target = Path::new(&paths[paths.len()-1]);
        if !target.exists() {
            println!("mv: target '{:?}' is not a directory", target.file_name());
            return;
        }
        for (i, p) in paths.iter().enumerate() {
            if i != paths.len() - 1{
                copy_dir(Path::new(p), target);
            }
        }
        for (i, p) in paths.iter().enumerate() {
            if i != paths.len() - 1{
                let path = Path::new(p);
                if path.is_dir() {
                    remove_dir_all(path);
                } else {
                    remove_file(path);
                }
            }
        }
        nonepaths.iter().for_each(|c| println!("mv: cannot stat '{}': No such file or directory", c));
    }else if args.len() == 1 {
        println!("mv: missing destination file operand after '{}'", args[0]);
    }else {
        println!("mv: missing file operand");
    }
}
