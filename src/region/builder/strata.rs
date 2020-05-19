use super::{Region, super::TileType};
use crate::planet::{REGION_WIDTH, REGION_HEIGHT, REGION_DEPTH};
use crate::utils::mapidx;

pub fn layer_cake(hm: &[u8], region: &mut Region) {
    // Clear it
    region.tile_types.iter_mut().for_each(|tt| *tt = TileType::Empty);

    // Build layered tiles
    for idx in 0..hm.len() {
        let x = idx % REGION_WIDTH as usize;
        let y = idx / REGION_WIDTH as usize;
        let altitude = hm[idx] as usize;

        // SMR at the very bottom
        region.tile_types[mapidx(x, y, 0)] = TileType::SemiMoltenRock;

        let mut z : usize = 0;
        /*while z < altitude {
            if x==0 || x==REGION_WIDTH as usize-1 || y==0 || y==REGION_HEIGHT as usize-1 {
                //region.tile_types[mapidx(x,y,z)] = TileType::SemiMoltenRock;
                region.tile_types[mapidx(x,y,z)] = TileType::Solid;
            } else {
                region.tile_types[mapidx(x,y,z)] = TileType::Empty;
                region.tile_types[mapidx(x,y,z)] = TileType::Solid;
            }
            z += 1;
        }*/

        // Next up is rock until the soil layer
        while z < usize::min(altitude, REGION_DEPTH as usize-20) {
            region.tile_types[mapidx(x,y,z)] = TileType::Solid;
            z += 1;
        }

        // Surface at z-1
        region.tile_types[mapidx(x,y,z-1)] = TileType::Floor;

        // Sky
        while z < REGION_DEPTH as usize {
            region.tile_types[mapidx(x,y,z)] = TileType::Empty;
            z += 1;
        }
    }
}