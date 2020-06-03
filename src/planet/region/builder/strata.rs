use super::{super::TileType, Region};
use crate::planet::{REGION_DEPTH, REGION_HEIGHT, REGION_WIDTH};
use crate::utils::mapidx;

pub fn build_strata() {
    
}

pub fn layer_cake(hm: &[u8], region: &mut Region) {
    // Clear it
    region
        .tile_types
        .iter_mut()
        .for_each(|tt| *tt = TileType::Empty);

    // Build layered tiles
    //let x = 4;
    for x in 0..REGION_WIDTH {
        for y in 0..REGION_HEIGHT {
            let altitude = hm[(y * REGION_WIDTH) + x] as usize;
            for z in 0..altitude {
                region.tile_types[mapidx(x, y, z)] = TileType::Solid;
            }
        }
    }
}
