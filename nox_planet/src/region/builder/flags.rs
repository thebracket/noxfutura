use crate::{mapidx, Region, TileType, REGION_DEPTH, REGION_HEIGHT, REGION_WIDTH};

pub fn set_flags(region: &mut Region) {
    for (idx, tt) in region.tile_types.clone().iter().enumerate() {
        match tt {
            TileType::SemiMoltenRock => region.set_flag(idx, Region::SOLID),
            TileType::Solid => region.set_flag(idx, Region::SOLID),
            TileType::TreeTrunk => region.set_flag(idx, Region::SOLID),
            TileType::Wall => region.set_flag(idx, Region::SOLID),
            TileType::Window => region.set_flag(idx, Region::SOLID),
            _ => {}
        }
    }

    for y in 0..REGION_HEIGHT {
        for x in 0..REGION_WIDTH {
            let mut blocked = false;
            for z in (0..REGION_DEPTH).rev() {
                let idx = mapidx(x, y, z);
                if !blocked {
                    region.set_flag(idx, Region::OUTSIDE);
                }
                if region.flag(idx, Region::SOLID) {
                    blocked = true;
                }
            }
        }
    }
}
