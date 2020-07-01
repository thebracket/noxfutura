use crate::components::prelude::*;
use std::collections::HashSet;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[uuid = "2b4c9d8c-a941-41a2-a850-342b2759c1ef"]
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
