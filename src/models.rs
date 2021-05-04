pub mod items {
    use std::fmt;
    use std::fmt::Formatter;

    #[derive(Debug)]
    pub struct TodoItem {
        pub name: String,
        pub completed: char,
    }

    impl TodoItem {
        pub fn new(name: String, completed: char) -> TodoItem {
            TodoItem{ name, completed }
        }
    }

    impl fmt::Display for TodoItem {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "[{}] - {}", self.completed, self.name)
        }
    }

    #[derive(Debug)]
    pub struct TodoList {
        pub list: Vec<TodoItem>
    }

    impl TodoList {
        pub fn new() -> TodoList {
            return TodoList{ list: Vec::new() }
        }

        pub fn add(&mut self, item: TodoItem) {
            self.list.push(item)
        }
    }
}
