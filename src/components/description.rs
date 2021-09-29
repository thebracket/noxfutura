pub struct Description {
    pub description: String,
}

impl Description {
    pub fn new<S: ToString>(name: S) -> Self {
        Self {
            description: name.to_string(),
        }
    }
}
