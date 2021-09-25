use crate::simulation::WORLD_WIDTH;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct PlanetLocation {
    pub x: usize,
    pub y: usize,
}

impl PlanetLocation {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn to_region_index(&self) -> usize {
        (self.y * WORLD_WIDTH) + self.x
    }
}
