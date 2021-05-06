use std::env;

use models::items::TodoItem;
use service::helpers;

mod models;
mod service;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let command = match arguments[1].as_str() {
        "get" => helpers::Commands::GetAll,
        "add" => helpers::Commands::Add(arguments[2].clone()),
        "compl" => helpers::Commands::Complete(arguments[2].clone()),
        "uncompl" => helpers::Commands::Uncomplete(arguments[2].clone()),
        _ => panic!("Unknown command")
    };

    let mut todos_list = Vec::new();
    todos_list.push(TodoItem::new("StaticTask".to_string(), ' '));
    todos_list.push(TodoItem::new("CompletedTask".to_string(), 'X'));

    match command {
        helpers::Commands::GetAll => {
            println!("{:#?}", todos_list)
        }
        helpers::Commands::Add(task_name) => {
            todos_list.push(TodoItem::new(task_name.clone(), ' '));
            println!("{:#?}", todos_list)
        }
        helpers::Commands::Complete(name) => {
            for item in todos_list.iter_mut() {
                if item.name == name {
                    item.complete();
                }
            }
            println!("{:#?}", &todos_list)
        }
        helpers::Commands::Uncomplete(name) => {
            for item in todos_list.iter_mut() {
                if item.name == name {
                    item.uncomplete();
                }
            }
            println!("{:#?}", &todos_list)
        }
    }
}
