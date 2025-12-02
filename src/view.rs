// src/view.rs
use crate::model::TodoItem;

pub fn print_help() {
    println!("Available commands:");
    println!("  add <title>    - Add a new task");
    println!("  list           - List all tasks");
    println!("  done <id>      - Mark a task as completed");
    println!("  help           - Show this help message");
}

pub fn print_tasks(tasks: &Vec<TodoItem>) {
    if tasks.is_empty() {
        println!("No tasks found. Go have a coffee! ☕");
        return;
    }

    println!("Todo List:");
    for item in tasks {
        let status = if item.completed { "[x]" } else { "[ ]" };
        println!("{} {} - {}", item.id, status, item.title);
    }
}

pub fn print_error(msg: &str) {
    eprintln!("Error: {}", msg);
}

pub fn print_message(msg: &str) {
    println!("{}", msg);
}
