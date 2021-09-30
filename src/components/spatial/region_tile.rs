use crate::simulation::{REGION_HEIGHT, REGION_WIDTH};
use bevy::math::Vec3;

pub struct RegionTileLocation {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl RegionTileLocation {
    pub fn new<N: Into<usize>>(x: N, y: N, z: N) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    /// Convert to a region tile index
    pub fn to_tile_index(&self) -> usize {
        (self.z * REGION_HEIGHT as usize * REGION_WIDTH as usize)
            + (self.y * REGION_WIDTH as usize)
            + self.x
    }

    /// Convert to a region-local render world-space
    pub fn to_world(&self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }
}
