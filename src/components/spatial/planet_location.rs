use crate::simulation::{REGION_HEIGHT, REGION_WIDTH, WORLD_WIDTH};
use bevy::math::Vec3;

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

    pub fn to_world(&self) -> Vec3 {
        Vec3::new(
            (self.x * REGION_WIDTH) as f32,
            (self.y * REGION_HEIGHT) as f32,
            0.0,
        )
    }
}
