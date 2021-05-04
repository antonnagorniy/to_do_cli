mod models;

use std::env;
use models::items::TodoItem;

fn main() {
    let get_command = String::from("get");
    let add_command = String::from("add");

    let arguments: Vec<String> = env::args().collect();
    let mut todos: Vec<TodoItem> = vec!(TodoItem{
        name: "New todo".to_string(),
        completed: ' '
    },
    TodoItem{
        name: "Comleted todo".to_string(),
        completed: 'X'
    });

    for x in arguments {
        if x == get_command {
           for y in &todos {
               println!("{}", y);
           }
        }else if x == add_command {
            todos.push(TodoItem{
                name: "Test".to_string(),
                completed: ' '
            })
        }
    }
}
