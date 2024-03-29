pub use crate::Planet;
use nox_spatial::{idxmap, mapidx, REGION_TILES_COUNT};
use serde::{Deserialize, Serialize};
mod tiletype;
pub use tiletype::*;
mod builder;
use bengine::geometry::*;
pub use builder::*;
use smallvec::SmallVec;
mod mining_map;
pub use mining_map::*;
mod lumber_map;
pub use lumber_map::*;
mod construction_map;
pub use construction_map::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct Region {
    pub world_idx: usize,
    pub tile_types: Vec<TileType>,
    pub material_idx: Vec<usize>,
    pub biome_info_idx: usize,
    pub biome_raw_idx: usize,
    pub revealed: Vec<bool>,
    pub water_level: Vec<u8>,
    flags: Vec<u16>,
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
            flags: vec![0u16; REGION_TILES_COUNT],
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
            flags: vec![0u16; REGION_TILES_COUNT],
        }
    }

    #[inline]
    pub fn flag(&self, idx: usize, flag: u16) -> bool {
        self.flags[idx] & flag > 0
    }

    pub fn set_flag(&mut self, idx: usize, flag: u16) {
        self.flags[idx] = self.flags[idx] | flag;
    }

    pub fn clear_flag(&mut self, idx: usize, flag: u16) {
        self.flags[idx] = self.flags[idx] & !flag;
    }

    pub fn reset_all_flags(&mut self) {
        self.flags.iter_mut().for_each(|f| *f = 0);
    }

    pub fn reset_all_flags_at(&mut self, idx: &usize) {
        self.flags[*idx] = 0;
    }

    pub fn reset_flags(&mut self, idx: usize) {
        self.flags[idx] = 0;
    }

    pub fn is_floor(&self, idx: usize) -> bool {
        match self.tile_types[idx] {
            TileType::Floor { .. } => true,
            _ => false,
        }
    }

    // Flags
    pub const SOLID: u16 = 1;
    pub const OUTSIDE: u16 = 2;
    pub const CONSTRUCTED: u16 = 4;
    pub const CAN_GO_NORTH: u16 = 8;
    pub const CAN_GO_SOUTH: u16 = 16;
    pub const CAN_GO_EAST: u16 = 32;
    pub const CAN_GO_WEST: u16 = 64;
    pub const CAN_GO_UP: u16 = 128;
    pub const CAN_GO_DOWN: u16 = 256;
    pub const CAN_STAND_HERE: u16 = 512;

    pub fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::<[(usize, f32); 10]>::new();

        let (x, y, z) = idxmap(idx);

        if self.flag(idx, Region::CAN_GO_NORTH) {
            exits.push((mapidx(x, y - 1, z), 1.0))
        }
        if self.flag(idx, Region::CAN_GO_SOUTH) {
            exits.push((mapidx(x, y + 1, z), 1.0))
        }
        if self.flag(idx, Region::CAN_GO_WEST) {
            exits.push((mapidx(x - 1, y, z), 1.0))
        }
        if self.flag(idx, Region::CAN_GO_EAST) {
            exits.push((mapidx(x + 1, y, z), 1.0))
        }
        if self.flag(idx, Region::CAN_GO_UP) {
            exits.push((mapidx(x, y, z + 1), 1.0));
        }
        if self.flag(idx, Region::CAN_GO_DOWN) {
            exits.push((mapidx(x, y, z - 1), 1.0));
        }

        exits
    }

    pub(crate) fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let (sx, sy, sz) = idxmap(idx1);
        let (ex, ey, ez) = idxmap(idx2);
        let pt1 = Point3::new(sx as i32, sy as i32, sz as i32);
        let pt2 = Point3::new(ex as i32, ey as i32, ez as i32);
        DistanceAlg::Pythagoras.distance3d(pt1, pt2)
    }
}
