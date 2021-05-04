pub mod items {
    use std::fmt;
    use std::fmt::Formatter;

    #[derive(Debug)]
    #[derive(Clone)]
    pub struct TodoItem {
        name: String,
        completed: char,
    }

    impl TodoItem {
        pub fn new(name: String, completed: char) -> TodoItem {
            TodoItem{ name, completed }
        }
        pub fn complete_item(&mut self){
            self.completed = 'X'
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

        pub fn get(&mut self, index: usize) -> &mut TodoItem {
            return &mut self.list[index];
        }
    }
}
