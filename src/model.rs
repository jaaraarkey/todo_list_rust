use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::BufReader;
use std::path::Path; // Added for serde_json::to_writer and from_reader

// src/model.rs

// We'll need to serialize these later, but for now let's just derive Debug and Clone.
// 'pub' means these structs and fields are accessible from other modules (like our future View).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    pub items: Vec<TodoItem>,
}

impl TodoList {
    // Constructor: Creates a new, empty TodoList
    pub fn new() -> TodoList {
        TodoList { items: Vec::new() }
    }

    // Adds a new item to the list
    // We take 'mut self' because we are modifying the list
    pub fn add(&mut self, title: String) {
        let id = self.items.len() as u64 + 1;
        let new_item = TodoItem {
            id,
            title,
            completed: false,
        };
        self.items.push(new_item);
    }

    // Returns a reference to the items (immutable)
    // We don't want the caller to mess with the vector directly
    pub fn get_all(&self) -> &Vec<TodoItem> {
        &self.items
    }

    // Marks an item as complete by ID
    // Returns true if found, false if not
    pub fn complete(&mut self, id: u64) -> bool {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.completed = true;
            return true;
        }
        false
    }

    // Save the list to a JSON file
    pub fn save(&self) -> Result<(), std::io::Error> {
        let file = File::create("db.json")?;
        serde_json::to_writer(file, &self)?;
        Ok(())
    }

    // Load the list from a JSON file
    pub fn load() -> Result<TodoList, std::io::Error> {
        if !Path::new("db.json").exists() {
            return Ok(TodoList::new());
        }
        let file = File::open("db.json")?;
        let reader = BufReader::new(file);
        let list = serde_json::from_reader(reader)?;
        Ok(list)
    }

    // Delete an item by ID
    // Returns true if found and deleted, false if not
    pub fn delete(&mut self, id: u64) -> bool {
        if let Some(index) = self.items.iter().position(|i| i.id == id) {
            self.items.remove(index);
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_list() {
        let mut list = TodoList::new();

        // Test Add
        list.add("Buy milk".to_string());
        list.add("Walk the dog".to_string());
        assert_eq!(list.get_all().len(), 2);
        assert_eq!(list.get_all()[0].title, "Buy milk");

        // Test Complete
        let success = list.complete(1);
        assert!(success);
        assert!(list.get_all()[0].completed);

        // Test Complete Non-existent
        let fail = list.complete(999);
        assert!(!fail);
    }
}
