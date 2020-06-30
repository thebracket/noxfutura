pub use crate::planet::{Planet, REGION_DEPTH, REGION_HEIGHT, REGION_TILES_COUNT, REGION_WIDTH};
use serde::{Deserialize, Serialize};
mod tiletype;
pub use tiletype::*;
mod builder;
pub use builder::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct Region {
    pub world_idx: usize,
    pub tile_types: Vec<TileType>,
    pub material_idx: Vec<usize>,
    pub biome_info_idx: usize,
    pub biome_raw_idx: usize,
    pub revealed: Vec<bool>,
    pub water_level: Vec<u8>,
    pub tree_id: Vec<usize>,
    pub vegetation_type_id: Vec<Option<usize>>,
    flags: Vec<u8>
}

impl Region {
    pub fn zeroed(world_idx: usize, planet: &Planet) -> Self {
        Self {
            world_idx,
            tile_types: vec![TileType::Empty; REGION_TILES_COUNT],
            biome_info_idx: planet.landblocks[world_idx].biome_idx,
            biome_raw_idx: planet.biomes[planet.landblocks[world_idx].biome_idx].biome_type,
            material_idx: vec![0; REGION_TILES_COUNT],
            revealed: vec![false; REGION_TILES_COUNT],
            water_level: vec![0; REGION_TILES_COUNT],
            tree_id: vec![0; REGION_TILES_COUNT],
            vegetation_type_id: vec![None; REGION_TILES_COUNT],
            flags: vec![0u8; REGION_TILES_COUNT],
        }
    }

    pub fn initial() -> Self {
        Self {
            world_idx: 0,
            tile_types: vec![TileType::Empty; REGION_TILES_COUNT],
            biome_info_idx: 0,
            biome_raw_idx: 0,
            material_idx: vec![0; REGION_TILES_COUNT],
            revealed: vec![false; REGION_TILES_COUNT],
            water_level: vec![0; REGION_TILES_COUNT],
            tree_id: vec![0; REGION_TILES_COUNT],
            vegetation_type_id: vec![None; REGION_TILES_COUNT],
            flags: vec![0u8; REGION_TILES_COUNT]
        }
    }

    pub fn flag(&self, idx: usize, flag: u8) -> bool {
        self.flags[idx] & flag > 0
    }

    pub fn set_flag(&mut self, idx: usize, flag: u8) {
        self.flags[idx] = self.flags[idx] | flag;
    }

    // Flags
    pub const SOLID : u8 = 1;
    pub const OUTSIDE : u8 = 2;
}
