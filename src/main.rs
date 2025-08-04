use std::env;
use std::process;

fn main() {
    let list_path = todo::parse_command(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing command: {err}");
        print_usage();
        process::exit(1);
    });

    println!("TODO list updated: {list_path}");
}

fn print_usage() {
    eprintln!("");
    eprintln!("Usage:");
    eprintln!("  todo [COMMAND [ARGUMENT]...]");
    eprintln!("");
    eprintln!("Commands:");
    eprintln!("  list");
    eprintln!("    print current TODO list (default command)");
    eprintln!("");
    eprintln!("  add TASK [POSITION]");
    eprintln!("    add a new task to the end (or insert at POSITION)");
    eprintln!("");
    eprintln!("  remove POSITION");
    eprintln!("    remove task at POSITION");
    eprintln!("");
    eprintln!("  move FROM_POSITION TO_POSITION");
    eprintln!("    move task from one position to another");
}
