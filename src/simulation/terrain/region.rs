use super::{PlanetLocation, TileType};
use crate::simulation::{CHUNKS_PER_REGION, REGION_TILES_COUNT};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum RegionStatus {
    NotLoaded,
    CreatingTiles,
    CreatedTiles,
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RegionRequirement {
    Unimportant,
    Peek,
    Required,
}

#[derive(Clone)]
pub struct Region {
    pub location: PlanetLocation,
    pub required: RegionRequirement,
    pub status: RegionStatus,
    pub requires_render_chunks: bool,

    pub chunks_loaded: Vec<bool>,
    pub tile_types: Vec<TileType>,
    pub material: Vec<usize>,
    pub revealed: Vec<bool>,
}

impl Region {
    pub fn new(location: PlanetLocation, required: RegionRequirement, requires_render_chunks: bool) -> Self {
        Self {
            location,
            required,
            status: RegionStatus::NotLoaded,
            tile_types: vec![TileType::Empty; REGION_TILES_COUNT],
            material: vec![0; REGION_TILES_COUNT],
            revealed: vec![false; REGION_TILES_COUNT],
            requires_render_chunks,
            chunks_loaded: vec![false; CHUNKS_PER_REGION],
        }
    }
}
