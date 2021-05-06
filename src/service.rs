pub mod helpers {
    pub enum Commands {
        GetAll,
        Add(String),
        Complete(String),
        Uncomplete(String),
        Remove(String),
    }
}