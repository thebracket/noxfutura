use bracket_geometry::prelude::Point;

#[derive(Clone)]
pub struct RiverStep {
    pub pos: Point,
}

#[derive(Clone)]
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
