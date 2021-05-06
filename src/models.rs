pub mod items {
    use std::fmt;
    use std::fmt::{Formatter};
    use std::borrow::Borrow;

    #[derive(Debug, Clone)]
    pub struct TodoItem {
        pub name: String,
        pub completed: char,
    }

    impl TodoItem {
        pub fn new(name: String, completed: char) -> TodoItem {
            TodoItem { name, completed }
        }

        pub fn complete(&mut self) {
            self.completed = 'X';
        }

        pub fn uncomplete(&mut self) {
            self.completed = ' ';
        }
    }

    impl fmt::Display for TodoItem {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "[{}] - {}", self.completed, self.name)
        }
    }

    #[derive(Debug)]
    pub struct TodoList {
        list: Vec<TodoItem>
    }

    impl TodoList {
        pub fn new() -> TodoList {
            return TodoList{ list: Vec::new() }
        }

        pub fn add(&mut self, item: TodoItem) {
            self.list.push(item)
        }

        pub fn get(&mut self, index: usize) -> &TodoItem {
            return self.list[index].borrow();
        }

        pub fn get_by_name(&self, name: String) -> Option<&TodoItem> {
            for todo in self.list.iter() {
                if todo.name == name {
                    return Some(todo);
                }
            };
            None
        }
    }
}
