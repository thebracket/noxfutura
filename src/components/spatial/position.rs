use crate::simulation::{REGION_DEPTH, REGION_HEIGHT, REGION_WIDTH};

use super::{PlanetLocation, RegionTileLocation};
use bevy::prelude::Vec3;

/// Represents a location in the world
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub region: PlanetLocation,
    pub tile: RegionTileLocation,
}

impl Position {
    /// Create with a specific region identifier and tile coordinates
    pub fn new(region: PlanetLocation, tile: RegionTileLocation) -> Self {
        Self { region, tile }
    }

    pub fn with_tile_coords<N: Into<usize>>(region: PlanetLocation, x: N, y: N, z: N) -> Self {
        Self {
            region,
            tile: RegionTileLocation::new(x, y, z),
        }
    }

    /// Convert to a region tile index
    pub fn to_tile_index(&self) -> usize {
        self.tile.to_tile_index()
    }

    /// Convert to render-space world coordinates
    pub fn to_world(&self) -> Vec3 {
        self.region.to_world() + self.tile.to_world()
    }

    /// Apply a tile offset and recalculate IDs as needed.
    /// Returns a new position.
    pub fn offset<N: Into<i32>>(&self, x:N, y:N, z:N) -> Self {
        let mut new_pos = (
            self.tile.x as i32 + x.into(),
            self.tile.y as i32 + y.into(),
            self.tile.z as i32 + z.into(),
        );
        let mut region = self.region;
        while new_pos.0 < 0 {
            region.x -= 1;
            new_pos.0 += REGION_WIDTH as i32;
        }
        while new_pos.0 > REGION_WIDTH as i32 -1 {
            region.x += 1;
            new_pos.0 -= REGION_WIDTH as i32;
        }
        while new_pos.1 < 0 {
            region.y -= 1;
            new_pos.1 += REGION_WIDTH as i32;
        }
        while new_pos.1 > REGION_HEIGHT as i32 -1 {
            region.y += 1;
            new_pos.1 -= REGION_WIDTH as i32;
        }
        if new_pos.2 < 0 {
            new_pos.2 = 0;
        }
        if new_pos.2 > REGION_DEPTH as i32 -1 {
            new_pos.2 = REGION_DEPTH as i32 - 1;
        }
        Self::with_tile_coords(region, new_pos.0 as usize, new_pos.1 as usize, new_pos.2 as usize)
    }
}
