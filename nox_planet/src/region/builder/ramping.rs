use crate::{ground_z, RampDirection, Region, TileType};
use nox_spatial::{mapidx, REGION_HEIGHT, REGION_WIDTH};


pub fn build_ramps(region: &mut Region) {
    for y in 1..REGION_HEIGHT - 1 {
        for x in 1..REGION_WIDTH - 1 {
            let z = ground_z(region, x, y);
            let idx = mapidx(x, y, z);
            if region.is_floor(idx) {
                if region.is_floor(mapidx(x, y - 1, z + 1)) {
                    region.tile_types[idx] = TileType::Ramp {
                        direction: RampDirection::NorthSouth,
                    };
                } else if region.is_floor(mapidx(x, y + 1, z + 1)) {
                    region.tile_types[idx] = TileType::Ramp {
                        direction: RampDirection::SouthNorth,
                    };
                } else if region.is_floor(mapidx(x + 1, y, z + 1)) {
                    region.tile_types[idx] = TileType::Ramp {
                        direction: RampDirection::WestEast,
                    };
                } else if region.is_floor(mapidx(x - 1, y, z + 1)) {
                    region.tile_types[idx] = TileType::Ramp {
                        direction: RampDirection::EastWest,
                    };
                }
            }
        }
    }
}
