pub mod todo_item {
    use std::fmt;
    use std::fmt::Formatter;

    #[derive(Debug)]
    pub struct TodoItem {
        pub name: String,
        pub completed: char,
    }

    impl fmt::Display for TodoItem {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "[{}] - {}", self.completed, self.name)
        }
    }
}
