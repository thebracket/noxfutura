use crate::components::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Initiative {
    pub initiative: i32,
    pub modifier: i32,
}

impl Initiative {
    pub fn new() -> Self {
        Self {
            initiative: 0,
            modifier: 0,
        }
    }
}
