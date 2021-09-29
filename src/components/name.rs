pub struct Name {
    pub name: String,
}

impl Name {
    pub fn new<S: ToString>(name: S) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
