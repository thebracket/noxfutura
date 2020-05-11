pub use crate::planet::{REGION_DEPTH, REGION_HEIGHT, REGION_WIDTH, REGION_TILES_COUNT, Planet};
mod tiletype;
pub use tiletype::TileType;
mod builder;
pub use builder::builder;

pub struct Region {
    pub world_idx : usize,
    pub tiles : Vec<TileType>,
    pub biome_info_idx : usize,
    pub biome_raw_idx : usize,
}

impl Region {
    pub fn zeroed(world_idx : usize, planet: &Planet) -> Self {
        Self {
            world_idx,
            tiles : vec![TileType::Empty; REGION_TILES_COUNT],
            biome_info_idx : planet.landblocks[world_idx].biome_idx,
            biome_raw_idx : planet.biomes[planet.landblocks[world_idx].biome_idx].biome_type
        }
    }
}