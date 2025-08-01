use std::env;
use std::process;

fn main() {
    let list_path = todo::parse_command(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing command: {err}");
        process::exit(1);
    });

    println!("TODO list updated: {list_path}");
}
