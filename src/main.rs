mod models;

use std::env;
use models::items::TodoItem;
use crate::models::items::TodoList;

fn main() {
    let get_command = String::from("get");
    let add_command = String::from("add");
    let complete_command = String::from("compl");

    let arguments: Vec<String> = env::args().collect();

    let mut todos = TodoList::new();

    for x in arguments {
        if x == get_command {
            let y = todos.get(0).clone();
            println!("{}", y);
        } else if x == add_command {
            todos.add(TodoItem::new("Test".to_string(), ' '))
        }else if x == complete_command {
            todos.get(0).complete_item()
        }
    }
}
