use crate::planet::{Region, TileType, REGION_WIDTH, REGION_HEIGHT, REGION_DEPTH};
use crate::utils::{ground_z, mapidx};

pub fn set_flags(region: &mut Region) {
    for (idx, tt) in region.tile_types.clone().iter().enumerate() {
        region.flags[idx] = 0u8;
        match tt {
            TileType::SemiMoltenRock => region.set_flag(idx, Region::SOLID),
            TileType::Solid => region.set_flag(idx, Region::SOLID),
            TileType::TreeTrunk => region.set_flag(idx, Region::SOLID),
            TileType::Wall => region.set_flag(idx, Region::SOLID),
            _ => {}
        }
    }

    for y in 0 .. REGION_HEIGHT {
        for x in 0 .. REGION_WIDTH {
            for z in ground_z(region, x, y)-1 .. REGION_DEPTH {
                region.set_flag(mapidx(x, y,z), Region::OUTSIDE);
            }
        }
    }
}