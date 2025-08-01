use std::env;

pub fn parse_command(
    mut args: impl Iterator<Item = String>,
) -> Result<String, &'static str> {
    args.next();

    let command = match args.next() {
        Some(arg) => arg,
        None => String::from("list"),
    };

    match command.as_str() {
        "list" => println!("Listing tasks..."),
        "add" => println!("Adding task..."),
        "remove" => println!("Removing task..."),
        "move" => println!("Moving task..."),
        _ => return Err("unrecognized command"),
    }

    //JMT TODO: change to list_path
    Ok(String::from(".todo"))
}
