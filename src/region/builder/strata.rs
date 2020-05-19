use super::{Region, super::TileType};
use crate::planet::{REGION_WIDTH, REGION_HEIGHT, REGION_DEPTH};
use crate::utils::mapidx;

pub fn layer_cake(hm: &[u8], region: &mut Region) {
    // Clear it
    region.tile_types.iter_mut().for_each(|tt| *tt = TileType::Empty);

    // Build layered tiles
    //let x = 4;
    for x in 0..REGION_WIDTH {
        for y in 0..REGION_HEIGHT {
            let altitude = hm[(y * REGION_WIDTH) + x] as usize;
            //for z in altitude..altitude+5 {
                region.tile_types[mapidx(x, y, altitude)] = TileType::Solid;
            //}
        }
    }
}