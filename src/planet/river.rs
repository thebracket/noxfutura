use bracket_geometry::prelude::Point;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct RiverStep {
    pub pos: Point,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct River {
    pub name: String,
    pub start: Point,
    pub steps: Vec<RiverStep>,
}

impl River {
    pub fn new() -> Self {
        Self{
            name : String::new(),
            start: Point::zero(),
            steps: Vec::new()
        }
    }
}
