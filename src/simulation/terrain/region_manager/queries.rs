use crate::simulation::{REGION_DEPTH, mapidx, terrain::{PlanetLocation, RegionStatus, TileType}};
use super::REGIONS;

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