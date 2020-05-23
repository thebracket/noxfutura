use super::{Region, super::TileType};
use crate::planet::{REGION_WIDTH, REGION_HEIGHT, REGION_DEPTH};
use crate::utils::mapidx;
use super::HeightMap;

pub fn layer_cake(hm: &HeightMap, region: &mut Region) {
    // Clear it
    region.tile_types.iter_mut().for_each(|tt| *tt = TileType::Empty);

    // Build layered tiles
    //let x = 4;
    for x in 0..REGION_WIDTH {
        for y in 0..REGION_HEIGHT {
            let altitude = hm[(y * REGION_WIDTH) + x] as usize;
            //println!("{}", altitude);
            for z in 0..altitude+32 {
                region.tile_types[mapidx(x, y, z)] = TileType::Solid;
            }
        }
    }
}