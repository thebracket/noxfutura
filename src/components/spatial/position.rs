use super::{PlanetLocation, RegionTileLocation};
use bevy::prelude::Vec3;

/// Represents a location in the world
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
}
