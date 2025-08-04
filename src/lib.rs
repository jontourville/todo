use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use chrono::NaiveDate;

#[derive(Clone)]
pub struct Task {
    pub name: String,
    pub due_date: NaiveDate,
    pub order: i32, // Used to store original order when sorting by date
}

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

    // Save the TODO list
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

    // Add a new task to the TODO list
    pub fn add(&mut self, name: &String, due_date: NaiveDate) {
        self.tasks.push(Task {
            name: name.clone(),
            due_date: due_date,
            order: 0,
        });
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

    let date_text = args.next().unwrap_or_default();
    let due_date = NaiveDate::parse_from_str(
        date_text.as_str(), 
        "%Y-%m-%d")
        .unwrap_or_else(|_| { NaiveDate::MAX });

    list.add(&name.unwrap(), due_date);
    list.print();
}

fn print_tasks(tasks: &Vec<Task>) {
    if tasks.len() == 0 {
        println!("(empty TODO list)");
        return;
    }

    println!("{:6} | {:10} | {}", "TASK #", "DUE DATE", "TASK");
    println!("==========================");

    for task in tasks.iter() {
        let mut due_date = String::new();
        if task.due_date != NaiveDate::MAX {
            due_date = task.due_date.format("%Y-%m-%d").to_string();
        }

        println!("{:6} | {:10} | {}",
            task.order,
            due_date,
            task.name
        );
    }
}

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  todo [COMMAND [ARGUMENT]...]");
    eprintln!("");
    eprintln!("Commands:");
    eprintln!("  list");
    eprintln!("    print current TODO list (default command)");
    eprintln!("");
    eprintln!("  date");
    eprintln!("    print current TODO list sorted by due date");
    eprintln!("");
    eprintln!("  add TASK [DUE DATE]");
    eprintln!("    add a new task to the end with an optional due date");
    eprintln!("    in the format YYYY-MM-DD");
    eprintln!("");
    eprintln!("  remove POSITION");
    eprintln!("    remove task at POSITION");
    eprintln!("");
    eprintln!("  move FROM_POSITION TO_POSITION");
    eprintln!("    move task from one position to another");
}
