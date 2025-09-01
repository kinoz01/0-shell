use std::io::Write;
pub fn run(args: Vec<String>) {
    if args.len() == 0 {
        return;
    }
    println!("{args:?}");
    args.iter().enumerate().for_each(|(i, c)| {
        if i != args.len() - 1 {
            print!("{c} ");
            std::io::stdout().flush();
        } else {
            println!("{c}");
        }
    });
}
