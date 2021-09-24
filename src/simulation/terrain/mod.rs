mod region;
mod tile_type;
pub use tile_type::*;
mod global_planet;
pub use global_planet::*;
mod strata;
use super::{mapidx, Planet, REGION_DEPTH, WORLD_WIDTH};
use lazy_static::*;
use parking_lot::RwLock;
pub use region::*;
use std::collections::HashMap;
mod region_loader;
pub use region_loader::*;
mod game_camera;
mod greedy;
mod region_chunks;
pub use game_camera::*;
mod chunk_iter;
use chunk_iter::*;
mod chunk_location;
pub use chunk_location::*;
mod change_batch;
pub use change_batch::*;

/// Call this after the raw files have loaded.
pub fn verify_strata() {
    use self::strata::StrataMaterials;
    PLANET_STORE.write().strata = Some(StrataMaterials::read());
}

pub fn set_global_planet(planet: Planet) {
    let planet_copy = planet.clone();
    PLANET_STORE.write().planet = Some(planet);
    PLANET_STORE.write().height_noise = Some(planet_copy.get_height_noise());
    PLANET_STORE.write().material_noise = Some(planet_copy.get_material_noise());
}

lazy_static! {
    static ref REGIONS: RwLock<Regions> = RwLock::new(Regions::new());
}

pub struct Regions {
    pub regions: HashMap<usize, Region>,
}

impl Regions {
    pub fn new() -> Self {
        Self {
            regions: HashMap::new(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct PlanetLocation {
    pub x: usize,
    pub y: usize,
}

impl PlanetLocation {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn to_region_index(&self) -> usize {
        (self.y * WORLD_WIDTH) + self.x
    }
}

pub fn spawn_playable_region(location: PlanetLocation) {
    let index = location.to_region_index();
    let mut region_lock = REGIONS.write();
    region_lock
        .regions
        .insert(index, Region::new(location, RegionRequirement::Required));
}

pub fn is_region_loaded(location: PlanetLocation) -> bool {
    let index = location.to_region_index();
    let region_lock = REGIONS.read();
    if let Some(region) = region_lock.regions.get(&index) {
        if region.status == RegionStatus::CreatedTiles {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

pub fn ground_z(region_id: PlanetLocation, x: usize, y: usize) -> usize {
    let index = region_id.to_region_index();
    let region_lock = REGIONS.read();
    if let Some(region) = region_lock.regions.get(&index) {
        if region.status == RegionStatus::CreatedTiles {
            let mut z = REGION_DEPTH - 1;
            let mut hit_ground = false;
            while !hit_ground {
                let idx = mapidx(x, y, z);
                if region.tile_types[idx] == TileType::Solid {
                    hit_ground = true;
                    z += 1;
                } else {
                    z -= 1;
                }
                if z == 1 {
                    hit_ground = true;
                }
            }
            return z;
        } else {
            return 0;
        }
    }
    0
}
