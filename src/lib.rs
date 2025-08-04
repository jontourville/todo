use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use chrono::prelude::*;

#[derive(Clone)]
pub struct Task {
    pub name: String,
    pub due_date: NaiveDate,
    pub order: usize, // Used to store original order when sorting by date
}

pub struct TodoList {
    pub path: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    // Load and return a todo list from a file
    pub fn load(path: &String) -> TodoList {
        let mut list = TodoList {
            path: path.clone(),
            tasks: Vec::new(),
        };

        for line in fs::read_to_string(path).unwrap_or_default().lines() {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            // Expected format is DATE,TASK NAME
            let (date_text, name) = line.split_once(',').unwrap_or_else(|| {
                ("", line)
            });

            let due_date = NaiveDate::parse_from_str(
                date_text, 
                "%Y-%m-%d")
                .unwrap_or_else(|_| { NaiveDate::MAX });

            list.tasks.push(Task {
                name: name.to_string(),
                due_date: due_date,
                order: 0,
            });
        }

        list
    }

    // Print all tasks in order
    pub fn print(&mut self) {
        self.update_order();
        print_tasks(&self.tasks);
    }

    // Print all tasks sorted by due date
    pub fn print_by_due_date(&mut self) {
        self.update_order();

        let mut tasks = self.tasks.clone();
        tasks.sort_by_key(|task| {task.due_date});

        print_tasks(&tasks);
    }

    // Save the todo list
    pub fn save(&self) -> Result<(), std::io::Error> {
        let mut file = File::create(&self.path)?;

        for task in self.tasks.iter() {
            writeln!(file, "{},{}",
                task.due_date.format("%Y-%m-%d").to_string(),
                task.name
            )?;
        }

        Ok(())
    }

    // Add a new task to the todo list
    pub fn add(&mut self, name: &String, due_date: NaiveDate) {
        self.tasks.push(Task {
            name: name.clone(),
            due_date: due_date,
            order: 0,
        });
    }

    // Remove a task from the todo list
    pub fn remove(&mut self, index: usize) {
        if index >= self.tasks.len() {
            eprintln!(
                "Error position must be between 1 and {}",
                self.tasks.len()
            );
            process::exit(1);
        }

        self.tasks.remove(index);
        println!("Removed task {}", index + 1);
    }

    // Move a task to a different position
    pub fn reorder(&mut self, from: usize, to: usize) {
        if from >= self.tasks.len() || to >= self.tasks.len() {
            eprintln!(
                "Error positions must be between 1 and {}",
                self.tasks.len()
            );
            process::exit(1);
        }

        let task = self.tasks.remove(from);
        self.tasks.insert(to, task);
        println!("Moved task {} to {}", from + 1, to + 1);
    }

    // Update the tasks' with their order in the Vec
    fn update_order(&mut self) {
        let mut order = 1;

        for task in self.tasks.iter_mut() {
            task.order = order;
            order += 1;
        }
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
        "date" => {
            list.print_by_due_date();
            is_modified = false;
        },
        "add" => parse_add(args, &mut list),
        "remove" => parse_remove(args, &mut list),
        "move" => parse_move(args, &mut list),
        "help" | "--help" | "-h" => print_usage(),
        _ => {
            eprintln!("Error unknown command: {command}");
            print_usage();
            process::exit(1);
        }
    }

    if is_modified {
        list.save().unwrap_or_else(|err| {
            eprintln!("Error saving todo list to {}", list.path);
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

    let date_text = args.next().unwrap_or_default();
    let due_date = NaiveDate::parse_from_str(
        date_text.as_str(), 
        "%Y-%m-%d")
        .unwrap_or_else(|_| { NaiveDate::MAX });

    list.add(&name.unwrap(), due_date);
    list.print();
}

fn parse_remove(
    mut args: impl Iterator<Item = String>,
    list: &mut TodoList,
) {
    let position = match args.next() {
        Some(num) => num,
        None => {
            eprintln!("Error no position provided to remove command");
            print_usage();
            process::exit(1);
        }
    };

    let position: usize = match usize::from_str_radix(position.as_str(), 10) {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error invalid position provided to remove command");
            process::exit(1);
        }
    };

    if position < 1 {
        eprintln!("Error position cannot be < 1");
        process::exit(1);
    }

    list.remove(position - 1);
}

fn parse_move(
    mut args: impl Iterator<Item = String>,
    list: &mut TodoList,
) {
    let from = match args.next() {
        Some(num) => num,
        None => {
            eprintln!("Error no from position provided to move command");
            print_usage();
            process::exit(1);
        }
    };

    let from: usize = match usize::from_str_radix(from.as_str(), 10) {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error invalid from position provided to move command");
            process::exit(1);
        }
    };

    let to = match args.next() {
        Some(num) => num,
        None => {
            eprintln!("Error no to position provided to move command");
            print_usage();
            process::exit(1);
        }
    };

    let to: usize = match usize::from_str_radix(to.as_str(), 10) {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error invalid to position provided to move command");
            process::exit(1);
        }
    };

    if from < 1 || to < 1 {
        eprintln!("Error positions cannot be < 1");
        process::exit(1);
    }

    list.reorder(from - 1, to - 1);
}

fn print_tasks(tasks: &Vec<Task>) {
    if tasks.len() == 0 {
        println!("(empty todo list)");
        return;
    }

    println!("{:6} | {:10} | {}", "TASK #", "DUE DATE", "TASK");
    println!("==========================");

    let default_color = "\x1b[0m";
    let overdue_color = "\x1b[0;31m";

    for task in tasks.iter() {
        let mut due_date = String::new();
        if task.due_date != NaiveDate::MAX {
            due_date = task.due_date.format("%Y-%m-%d").to_string();
        }

        if task.due_date <= Local::now().date_naive() {
            print!("{}", overdue_color);
        }

        println!("{:6} | {:10} | {}{}",
            task.order,
            due_date,
            task.name,
            default_color,
        );
    }
}

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  todo [COMMAND [ARGUMENT]...]");
    eprintln!("");
    eprintln!("Commands:");
    eprintln!("  list");
    eprintln!("    print current todo list (default command)");
    eprintln!("");
    eprintln!("  date");
    eprintln!("    print current todo list sorted by due date");
    eprintln!("");
    eprintln!("  add TASK [DUE DATE]");
    eprintln!("    add a new task to the end with an optional due date");
    eprintln!("    in the format YYYY-MM-DD");
    eprintln!("");
    eprintln!("  remove POSITION");
    eprintln!("    remove task at POSITION");
    eprintln!("");
    eprintln!("  move FROM TO");
    eprintln!("    move task from one position to another");
}
