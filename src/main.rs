mod models;

use std::env;
use models::items::TodoItem;
use crate::models::items::TodoList;

fn main() {
    let get_command = String::from("get");
    let add_command = String::from("add");

    let arguments: Vec<String> = env::args().collect();

    let mut todos = TodoList::new();

    for x in arguments {
        if x == get_command {
            for y in &todos.list {
                println!("{}", y);
            }
        } else if x == add_command {
            todos.add(TodoItem::new("Test".to_string(), ' '))
        }
    }
}
