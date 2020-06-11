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
        }
    }
}
