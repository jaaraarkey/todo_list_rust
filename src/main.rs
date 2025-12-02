mod model;
mod view;

use model::TodoList;
use std::env;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Initialize our Model (Load from file if exists)
    let mut todo_list = TodoList::load().unwrap_or_else(|e| {
        view::print_error(&format!("Failed to load data: {}", e));
        TodoList::new()
    });

    // If no command provided, show help
    if args.len() < 2 {
        view::print_help();
        return Ok(());
    }

    // The command is the second argument (index 1)
    let command = &args[1];

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                view::print_error("Please provide a title for the task");
                return Ok(());
            }
            let title = &args[2];
            todo_list.add(title.to_string());
            todo_list.save()?;
            view::print_message(&format!("Task added: {}", title));
        }
        "list" => {
            view::print_tasks(todo_list.get_all());
        }
        "done" => {
            if args.len() < 3 {
                view::print_error("Please provide the ID of the task to complete");
                return Ok(());
            }
            match args[2].parse::<u64>() {
                Ok(id) => {
                    if todo_list.complete(id) {
                        todo_list.save()?;
                        view::print_message(&format!("Task {} marked as complete", id));
                    } else {
                        view::print_error("Task not found");
                    }
                }
                Err(_) => view::print_error("Invalid ID provided"),
            }
        }
        "help" => {
            view::print_help();
        }
        _ => {
            view::print_error("Unknown command");
            view::print_help();
        }
    }

    Ok(())
}
