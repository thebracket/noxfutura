use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Serialize, Deserialize)]
pub struct JobsBoard {
    designated_trees: HashSet<usize>,
}

impl JobsBoard {
    pub fn new() -> Self {
        Self {
            designated_trees : HashSet::new()
        }
    }

    pub fn get_trees(&self) -> &HashSet<usize> {
        &self.designated_trees
    }

    pub fn set_tree(&mut self, id: usize) {
        self.designated_trees.insert(id);
    }

    pub fn remove_tree(&mut self, id: &usize) {
        self.designated_trees.remove(id);
    }
}