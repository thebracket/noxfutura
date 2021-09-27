use super::REGIONS;
use crate::simulation::{
    idxmap, mapidx,
    terrain::{PlanetLocation, RegionStatus, TileType},
    REGION_DEPTH,
};

/// Returns true when a region has reached the "TilesCreated" stage---it
/// can be queried for tile content. Prettying hasn't occurred yet,
/// neither has render creation.
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

/// Checks a set of regions for "TileCreated" status---ready to be queried.
/// Prettying hasn't occurred, neither has render creation.
pub fn are_regions_loaded(locations: &[PlanetLocation]) -> bool {
    let mut loaded = true;
    let region_lock = REGIONS.read();
    locations
        .iter()
        .map(|loc| loc.to_region_index())
        .for_each(|idx| {
            if let Some(region) = region_lock.regions.get(&idx) {
                if region.status != RegionStatus::CreatedTiles {
                    loaded = false;
                }
            }
        });
    loaded
}

/// Scans downwards from the sky, returning the first tile in a region
/// that is solid, and then moving up one tile - telling you the top-most
/// tile on which an entity can stand.
pub fn ground_z(region_id: PlanetLocation, x: usize, y: usize) -> usize {
    let index = region_id.to_region_index();
    let region_lock = REGIONS.read();
    if let Some(region) = region_lock.regions.get(&index) {
        if region.status == RegionStatus::CreatedTiles {
            let mut z = REGION_DEPTH - 1;
            let mut hit_ground = false;
            while !hit_ground {
                let idx = mapidx(x, y, z);
                if region.tile_types[idx] == TileType::Floor {
                    hit_ground = true;
                } else if region.tile_types[idx] == TileType::Solid {
                    hit_ground = true;
                    z += 1; // Move up one
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

/// Returns true if a tile is solid - cannot be walked into.
pub fn is_tile_solid(region_id: PlanetLocation, tile_idx: usize) -> bool {
    let index = region_id.to_region_index();
    let region_lock = REGIONS.read();
    if let Some(region) = region_lock.regions.get(&index) {
        match region.tile_types[tile_idx] {
            TileType::Solid => true,
            TileType::SemiMoltenRock => true,
            _ => false,
        }
    } else {
        false
    }
}

/// Returns true if a tile is a floor or has a solid tile underneath it.
pub fn is_tile_floor(region_id: PlanetLocation, tile_idx: usize) -> bool {
    let index = region_id.to_region_index();
    let region_lock = REGIONS.read();
    if let Some(region) = region_lock.regions.get(&index) {
        match region.tile_types[tile_idx] {
            TileType::Floor => true,
            TileType::Empty => {
                let (x, y, z) = idxmap(tile_idx);
                if z > 1 {
                    let below_idx = mapidx(x, y, z - 1);
                    is_tile_solid(region_id, below_idx)
                } else {
                    false // Bottom of map
                }
            }
            _ => false,
        }
    } else {
        false
    }
}

pub fn get_material_idx(region_id: PlanetLocation, tile_idx: usize) -> usize {
    let index = region_id.to_region_index();
    let region_lock = REGIONS.read();
    if let Some(region) = region_lock.regions.get(&index) {
        region.material[tile_idx]
    } else {
        0
    }
}
