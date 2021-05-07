use std::env;

use models::items::TodoItem;
use service::helpers;

use crate::models::items::TodoList;

mod models;
mod service;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let command = match arguments[1].as_str() {
        "get" => helpers::Commands::GetAll,
        "add" => helpers::Commands::Add(arguments[2].clone()),
        "compl" => helpers::Commands::Complete(arguments[2].clone()),
        "uncompl" => helpers::Commands::Uncomplete(arguments[2].clone()),
        "rem" => helpers::Commands::Remove(arguments[2].clone()),
        _ => panic!("Unknown command")
    };

    let mut todos_list = TodoList::new();
    todos_list.push(TodoItem::new("StaticTask".to_string(), ' '));
    todos_list.push(TodoItem::new("CompletedTask".to_string(), 'X'));
    todos_list.push(TodoItem::new("TaskDelete".to_string(), 'X'));

    match command {
        helpers::Commands::GetAll => {
            println!("{}", todos_list)
        }
        helpers::Commands::Add(task_name) => {
            todos_list.push(TodoItem::new(task_name, ' '));
            println!("{}", todos_list)
        }
        helpers::Commands::Complete(name) => {
            let done = todos_list.done(name.clone());
            if done {
                println!("{}", &todos_list)
            } else {
                println!("Task {} not found.", name)
            }
        }
        helpers::Commands::Uncomplete(name) => {
            let undone = todos_list.undone(name.clone());
            if undone {
                println!("{}", &todos_list)
            } else {
                println!("Task {} not found.", name)
            }
        }
        helpers::Commands::Remove(name) => {
            let removed = todos_list.remove(name.clone());
            if removed {
                println!("{} deleted.", name.clone());
                println!("{}", &todos_list);
            } else {
                println!("Task {} not found.", name)
            }
        }
    }
}
