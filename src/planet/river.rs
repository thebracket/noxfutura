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
