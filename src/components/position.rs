use crate::simulation::{REGION_DEPTH, REGION_HEIGHT, REGION_WIDTH, terrain::PlanetLocation};
use bevy::prelude::Vec3;

pub struct Position {
    pub region: PlanetLocation,
    pub x : usize,
    pub y : usize,
    pub z : usize,
}

impl Position {
    pub fn new(region: PlanetLocation, x: usize, y: usize, z: usize) -> Self {
        Self {
            region, x, y, z
        }
    }

    pub fn to_tile_index(&self) -> usize {
        (self.z * REGION_HEIGHT as usize * REGION_WIDTH as usize) + (self.y * REGION_WIDTH as usize) + self.x
    }

    pub fn to_world(&self) -> Vec3 {
        let (x,y,z) = self.region.to_world();
        Vec3::new(
            x + self.x as f32,
            y + self.y as f32,
            z + self.z as f32,
        )
    }
}