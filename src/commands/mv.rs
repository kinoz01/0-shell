use crate::commands::*;


pub fn run(args: &[String]) {
    cp::run(args);
    let mut new_args = Vec::with_capacity(args.len());
    new_args.push("-r".to_string());
    new_args.extend(args[..args.len() - 1].to_vec());
    rm::run(&new_args);
}
