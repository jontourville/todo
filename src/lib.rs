use std::fs;

pub struct Task {
    pub name: String,
    pub order: i32,
}

pub fn parse_command(
    mut args: impl Iterator<Item = String>,
) -> Result<String, &'static str> {
    let list_path = String::from(".todo");

    args.next();
    let command = match args.next() {
        Some(arg) => arg,
        None => String::from("list"),
    };

    match command.as_str() {
        "list" => list_tasks(&list_path),
        "add" => println!("Adding task..."),
        "remove" => println!("Removing task..."),
        "move" => println!("Moving task..."),
        _ => return Err("unrecognized command"),
    }

    Ok(list_path)
}

fn load_tasks(list_path: &String) -> Vec<String> {
    fs::read_to_string(list_path)
        .unwrap_or_default()
        .lines()
        .map(String::from)
        .collect()
}

fn list_tasks(list_path: &String) {
    let mut i = 1;
    let tasks = load_tasks(list_path);

    println!("TODO List:");

    if tasks.len() == 0 {
        println!("    (empty)");
    }

    for task in tasks.iter() {
        println!("{:5}. {}", i, task);
        i = i + 1;
    }
}
