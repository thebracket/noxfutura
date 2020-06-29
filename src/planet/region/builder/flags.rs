use crate::planet::{Region, TileType};

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
}