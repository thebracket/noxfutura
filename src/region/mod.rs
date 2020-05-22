pub use crate::planet::{Planet, REGION_DEPTH, REGION_HEIGHT, REGION_TILES_COUNT, REGION_WIDTH};
mod tiletype;
pub use tiletype::TileType;
mod builder;
pub use builder::builder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Region {
    pub world_idx: usize,
    pub tile_types: Vec<TileType>,
    pub biome_info_idx: usize,
    pub biome_raw_idx: usize,
}

impl Region {
    pub fn zeroed(world_idx: usize, planet: &Planet) -> Self {
        Self {
            world_idx,
            tile_types: vec![TileType::Empty; REGION_TILES_COUNT],
            biome_info_idx: planet.landblocks[world_idx].biome_idx,
            biome_raw_idx: planet.biomes[planet.landblocks[world_idx].biome_idx].biome_type,
        }
    }
}
