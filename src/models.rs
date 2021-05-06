pub mod items {
    use std::fmt;
    use std::fmt::Formatter;

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
        list: Vec<TodoItem>,
    }

    impl fmt::Display for TodoList {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let mut result: String = "".to_string();
            for item in self.list.iter() {
                result.push_str(format!("[{}] - {}\n", item.completed, item.name).as_str());
            }
            write!(f, "{}", result)
        }
    }

    impl TodoList {
        pub fn new() -> TodoList {
            return TodoList { list: Vec::new() };
        }

        pub fn push(&mut self, item: TodoItem) {
            self.list.push(item)
        }

        pub fn remove(&mut self, name: String) -> bool {
            let mut done = false;
            let mut index: usize = 0;
            for (indx, item) in self.list.iter().enumerate() {
                if item.name == name {
                    index = indx;
                    done = true;
                }
            }

            if done {
                self.list.remove(index);
            }

            return done;
        }

        pub fn done(&mut self, name: String) -> bool {
            let mut completed = false;
            for item in self.list.iter_mut() {
                if item.name == name {
                    item.complete();
                    completed = true;
                }
            }
            return completed;
        }

        pub fn undone(&mut self, name: String) -> bool {
            let mut completed = false;
            for item in self.list.iter_mut() {
                if item.name == name {
                    item.uncomplete();
                    completed = true;
                }
            }
            return completed;
        }
    }
}
