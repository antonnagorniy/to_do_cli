mod models;

use std::env;
use models::items::TodoItem;
use crate::models::items::TodoList;
use std::ops::Add;

const GET_COMMAND: &str = "get";
const ADD_COMMAND: &str = "add";
const COMPLETE_COMMAND: &str = "compl";

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let arg = arguments[1].clone();
    let mut todos_list = TodoList::new();
    todos_list.add(TodoItem::new("StaticTask".to_string(), ' '));

    match arg.as_str() {
        GET_COMMAND => {
            let name = arguments[2].clone();
            let option = todos_list.get_by_name(name.clone());
            println!("{}",
                     option.expect("There is no such task: "
                         .to_string()
                         .add(&name)
                         .as_str()));
        }
        ADD_COMMAND => {
            let task_name = arguments[2].clone();
            todos_list.add(TodoItem::new(task_name.clone(), ' '));
            let option = todos_list.get_by_name(task_name.clone());
            println!("{}",
                     option.expect("There is no such task: "
                         .to_string()
                         .add(&task_name)
                         .as_str()));
        }
        COMPLETE_COMMAND => {
            todos_list.get(0).complete_item();
        }
        _ => {
            println!("Unknown command!")
        }
    }
}
