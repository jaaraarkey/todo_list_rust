mod model;
mod view;

use model::TodoList;
use std::env;

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Initialize our Model (Note: This creates a new empty list every time for now!)
    let mut todo_list = TodoList::new();

    // If no command provided, show help
    if args.len() < 2 {
        view::print_help();
        return;
    }

    // The command is the second argument (index 1)
    let command = &args[1];

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                view::print_error("Please provide a title for the task");
                return;
            }
            // Join all remaining arguments so "Buy milk" works without quotes if they want
            // But for simplicity, let's just take the 3rd arg for now, or join them.
            // Let's stick to simple: cargo run add "Buy milk"
            let title = &args[2];
            todo_list.add(title.to_string());
            view::print_message(&format!("Task added: {}", title));
        }
        "list" => {
            view::print_tasks(todo_list.get_all());
        }
        "done" => {
            if args.len() < 3 {
                view::print_error("Please provide the ID of the task to complete");
                return;
            }
            match args[2].parse::<u64>() {
                Ok(id) => {
                    if todo_list.complete(id) {
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
}
