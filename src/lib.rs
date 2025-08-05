use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use chrono::prelude::*;
use std::cmp::Ordering;

#[derive(Clone, Eq)]
pub struct Task {
    pub name: String,
    pub due_date: NaiveDate,
    pub order: usize,
}

pub struct TodoList {
    pub path: String,
    pub tasks: Vec<Task>,
}

// Order tasks by due date when sorting
impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.due_date
            .cmp(&other.due_date)
            .then(self.order.cmp(&other.order))
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.due_date == other.due_date && self.order == other.order
    }
}

impl TodoList {
    // Load and return a todo list from a file
    pub fn load(path: String) -> TodoList {
        let mut list = TodoList {
            path: path.clone(),
            tasks: Vec::new(),
        };

        let mut order: usize = 1;

        // Read in the todo list, initializing an empty list if the
        // file doesn't exist or is empty
        for line in fs::read_to_string(path).unwrap_or_default().lines() {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            // Expected format is DATE,TASK NAME
            // Order is determined by line number and not stored in the file
            let (date_text, name) = line.split_once(',').unwrap_or_else(|| {
                ("", line)
            });

            // Nonexistent or invalid due dates are interpreted as the max date
            // so they appear at the end when sorting by date
            let due_date = NaiveDate::parse_from_str(
                date_text, 
                "%Y-%m-%d")
                .unwrap_or_else(|_| { NaiveDate::MAX });

            list.tasks.push(Task {
                name: name.to_string(),
                due_date: due_date,
                order: order,
            });

            order += 1;
        }

        list
    }

    // Print all tasks in order
    pub fn print(&mut self) {
        print_tasks(&self.tasks);
    }

    // Print all tasks sorted by due date
    pub fn print_by_due_date(&mut self) {
        let mut tasks = self.tasks.clone();
        tasks.sort();
        print_tasks(&tasks);
    }

    // Save the todo list to a file
    pub fn save(&self) -> Result<(), std::io::Error> {
        let mut file = File::create(&self.path)?;

        for task in self.tasks.iter() {
            writeln!(file, "{},{}",
                task.due_date.format("%Y-%m-%d").to_string(),
                task.name
            )?;
        }

        println!("Saved todo list to: {}", self.path);
        Ok(())
    }

    // Add a new task to the todo list
    pub fn add(&mut self, name: String, due_date: NaiveDate) {
        self.tasks.push(Task {
            name: name.clone(),
            due_date: due_date,
            order: self.tasks.len() + 1,
        });

        println!("Added task {}", self.tasks.len());
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
        self.update_order();
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
        self.update_order();
        println!("Moved task {} to {}", from + 1, to + 1);
    }

    // Update all tasks'order based on their position in the Vec
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
    // Use the current directory to load/save the todo list
    // to support per-project lists
    let path = String::from(".todo");

    let mut list = TodoList::load(path);
    let mut is_modified = false;

    // Parse the command argument, defaulting to "list"
    args.next();
    let command = match args.next() {
        Some(arg) => arg,
        None => String::from("list"),
    };

    match command.as_str() {
        "list" => list.print(),

        "date" => list.print_by_due_date(),

        "add" => {
            parse_add(args, &mut list);
            is_modified = true;
        },

        "remove" => {
            parse_remove(args, &mut list);
            is_modified = true;
        },

        "move" => {
            parse_move(args, &mut list);
            is_modified = true;
        },

        "help" | "--help" | "-h" => print_usage(),

        _ => {
            eprintln!("Error unknown command: {command}");
            print_usage();
            process::exit(1);
        }
    }

    // Only write to disk if the command modified the todo list
    if is_modified {
        list.save().unwrap_or_else(|err| {
            eprintln!("Error saving todo list to {}", list.path);
            eprintln!("{err}");
            process::exit(1);
        });
    }
}

// Parse the add command's arguments and add the task
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

    // Currently, invalid or nonexistent due dates are interpreted
    // as max date for sorting
    // TODO: handle invalid dates separately and error out
    let date_text = args.next().unwrap_or_default();
    let due_date = NaiveDate::parse_from_str(
        date_text.as_str(), 
        "%Y-%m-%d")
        .unwrap_or_else(|_| { NaiveDate::MAX });

    list.add(name.unwrap(), due_date);
}

// Parse the remove command's arguments and remove the task
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

// Parse the move command's arguments and move the task to a new position
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

// Print all tasks in the order provided
fn print_tasks(tasks: &Vec<Task>) {
    if tasks.len() == 0 {
        println!("(empty todo list)");
        return;
    }

    println!("{:6} | {:10} | {}", "TASK #", "DUE DATE", "TASK");
    println!("==========================");

    // Print tasks past their due date in red
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

// Print program usage statement
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

// TODO: more extensive tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_empty() {
        let list = TodoList::load("nonexistent_file_path".to_string());
        assert!(list.tasks.is_empty());
    }

    #[test]
    fn add_one() {
        let mut list = TodoList::load("nonexistent_file_path".to_string());
        list.add("Task A".to_string(), NaiveDate::MAX);
        assert!(list.tasks.len() == 1);
    }

    #[test]
    fn remove_one() {
        let mut list = TodoList::load("nonexistent_file_path".to_string());
        list.add("Task A".to_string(), NaiveDate::MAX);
        list.remove(0);
        assert!(list.tasks.is_empty());
    }
}
