use crate::{Region, TileType};
use nox_spatial::{mapidx, REGION_DEPTH};

pub fn ground_z(region: &Region, x: usize, y: usize) -> usize {
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

    z
}
