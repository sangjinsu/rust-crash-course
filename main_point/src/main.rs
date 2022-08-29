use std::env::args;
use std::process::exit;

fn main() {
    let arg: String = args().
        collect::<Vec<String>>().
        iter().
        nth(1).
        unwrap_or_else(
            || {
                println!("need args");
                exit(1)
            }
        ).to_owned();

    inspect(&arg);
}

fn inspect(arg: &String) {
    if arg.ends_with("s") {
        println!("{} is plural", arg);
    } else {
        println!("{} is singular", arg);
    }
}
