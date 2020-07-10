use crate::{mapidx, Region, TileType, REGION_DEPTH, REGION_HEIGHT, REGION_WIDTH};

pub fn set_flags(region: &mut Region) {
    for (idx, tt) in region.tile_types.clone().iter().enumerate() {
        match tt {
            TileType::SemiMoltenRock => region.set_flag(idx, Region::SOLID),
            TileType::Solid => region.set_flag(idx, Region::SOLID),
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

    for y in 0..REGION_HEIGHT {
        for x in 0..REGION_WIDTH {
            for z in (0..REGION_DEPTH).rev() {
                let idx = mapidx(x, y, z);
                if y>0 && region.flag(idx - REGION_WIDTH, Region::SOLID) { region.set_flag(idx, Region::CAN_GO_NORTH) }
                if y<REGION_HEIGHT-1 && region.flag(idx + REGION_WIDTH, Region::SOLID) { region.set_flag(idx, Region::CAN_GO_SOUTH) }
                if x<REGION_WIDTH-1 && region.flag(idx +1, Region::SOLID) { region.set_flag(idx, Region::CAN_GO_EAST) }
                if x>0 && region.flag(idx -1, Region::SOLID) { region.set_flag(idx, Region::CAN_GO_WEST) }
                // TODO: Handle stairs and ramps
            }
        }
    }
}
