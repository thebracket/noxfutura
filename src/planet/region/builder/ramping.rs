use crate::planet::{Region, REGION_WIDTH, REGION_HEIGHT, TileType, RampDirection};
use crate::utils::{mapidx, ground_z};

pub fn build_ramps(region: &mut Region) {
    for y in 1..REGION_HEIGHT-1 {
        for x in 1..REGION_WIDTH-1 {
            let z = ground_z(region, x, y);
            let idx = mapidx(x, y, z);
            if region.tile_types[idx] == TileType::Floor {
                if region.tile_types[mapidx(x, y-1, z+1)] == TileType::Floor {
                    region.tile_types[idx] = TileType::Ramp{direction: RampDirection::NorthSouth};
                } else if region.tile_types[mapidx(x, y+1, z+1)] == TileType::Floor {
                    region.tile_types[idx] = TileType::Ramp{direction: RampDirection::SouthNorth};
                } else if region.tile_types[mapidx(x+1, y, z+1)] == TileType::Floor {
                    region.tile_types[idx] = TileType::Ramp{direction: RampDirection::EastWest};
                } else if region.tile_types[mapidx(x-1, y, z+1)] == TileType::Floor {
                    region.tile_types[idx] = TileType::Ramp{direction: RampDirection::WestEast};
                }
            }
        }
    }
}