use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process;

#[derive(Debug)]
pub struct Task {
    pub name: String,
    pub order: i32,
}

#[derive(Debug)]
pub struct TodoList {
    pub path: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    // Load and return a TODO list from a file
    pub fn load(path: &String) -> TodoList {
        let mut list = TodoList {
            path: path.clone(),
            tasks: Vec::new(),
        };

        let mut order: i32 = 1;

        for line in fs::read_to_string(path).unwrap_or_default().lines() {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            list.tasks.push(Task {
                name: line.to_string(),
                order: order,
            });

            order += 1;
        }

        list
    }

    // Print all tasks
    pub fn print(&self) {
        if self.tasks.len() == 0 {
            println!("    (empty)");
        }

        for task in self.tasks.iter() {
            println!("{:5}. {}", task.order, task.name);
        }
    }

    // Save the TODO list
    pub fn save(&self) -> Result<(), std::io::Error> {
        let mut file = File::create(&self.path)?;

        for task in self.tasks.iter() {
            writeln!(file, "{}", task.name)?;
        }

        Ok(())
    }

    // Add a new task to the TODO list
    pub fn add(&mut self, name: &String) {
        let order = match self.tasks.last() {
            Some(task) => task.order + 1,
            None => 1,
        };

        self.tasks.push(Task {
            name: name.clone(),
            order: order,
        });
    }
}

// Parse the program arguments and perform the specified command
pub fn parse_command(mut args: impl Iterator<Item = String>) {
    let path = String::from(".todo");

    let mut list = TodoList::load(&path);
    let mut is_modified = true;

    args.next();
    let command = match args.next() {
        Some(arg) => arg,
        None => String::from("list"),
    };

    match command.as_str() {
        "list" => {
            list.print();
            is_modified = false;
        },
        "add" => parse_add(args, &mut list),
        "remove" => println!("Removing task..."),
        "move" => println!("Moving task..."),
        "help" | "--help" | "-h" => print_usage(),
        _ => {
            eprintln!("Error unknown command: {command}");
            print_usage();
            process::exit(1);
        }
    }

    if is_modified {
        list.save().unwrap_or_else(|err| {
            eprintln!("Error saving TODO list to {}", list.path);
            eprintln!("{err}");
            process::exit(1);
        });
    }
}

fn parse_add(
    mut args: impl Iterator<Item = String>,
    list: &mut TodoList,
) {
    let name = args.next();
    if name.is_none() {
        eprintln!("Error no task provided to add command");
        print_usage();
        process::exit(1);
    }

    list.add(&name.unwrap());
    list.print();
}

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  todo [COMMAND [ARGUMENT]...]");
    eprintln!("");
    eprintln!("Commands:");
    eprintln!("  list");
    eprintln!("    print current TODO list (default command)");
    eprintln!("");
    eprintln!("  add TASK");
    eprintln!("    add a new task to the end");
    eprintln!("");
    eprintln!("  remove POSITION");
    eprintln!("    remove task at POSITION");
    eprintln!("");
    eprintln!("  move FROM_POSITION TO_POSITION");
    eprintln!("    move task from one position to another");
}
