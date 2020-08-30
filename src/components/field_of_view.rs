use crate::components::prelude::*;
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FieldOfView {
    pub is_dirty: bool,
    pub radius: usize,
    pub visible_tiles: HashSet<usize>,
}

impl FieldOfView {
    pub fn new(radius: usize) -> Self {
        Self {
            radius,
            visible_tiles: HashSet::new(),
            is_dirty: true,
        }
    }
}
