use crate::{Region, StairsType, TileType};
use nox_spatial::*;

pub fn set_flags(region: &mut Region) {
    // Set the solid flag
    for (idx, tt) in region.tile_types.clone().iter().enumerate() {
        match tt {
            TileType::SemiMoltenRock => region.set_flag(idx, Region::SOLID),
            TileType::Solid => region.set_flag(idx, Region::SOLID),
            TileType::Wall => region.set_flag(idx, Region::SOLID),
            TileType::Window => region.set_flag(idx, Region::SOLID),
            TileType::TreeFoliage{..} => region.set_flag(idx, Region::SOLID),
            TileType::TreeTrunk{..} => region.set_flag(idx, Region::SOLID),
            _ => {}
        }
    }

    // Figure out which tiles are outdoors
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

    // Where can a non-flying beastie (most of them) stand?
    for z in 0..REGION_DEPTH {
        for y in 0..REGION_HEIGHT {
            for x in 0..REGION_WIDTH {
                let idx = mapidx(x, y, z);
                let mut can_stand = false;
                if !region.flag(idx, Region::SOLID) {
                    match region.tile_types[idx] {
                        TileType::Floor{..} | TileType::Ramp { .. } | TileType::Stairs { .. } => {
                            can_stand = true;
                        }
                        _ => {}
                    }
                    if z < REGION_DEPTH - 1 {
                        let down_idx = mapidx(x, y, z + 1);
                        match region.tile_types[down_idx] {
                            TileType::Solid | TileType::Ramp { .. } => {
                                can_stand = true;
                            }
                            TileType::Stairs { direction } => match direction {
                                StairsType::Up | StairsType::UpDown => {
                                    can_stand = true;
                                }
                                _ => {}
                            },
                            _ => {}
                        }
                    }
                }
                if can_stand {
                    region.set_flag(idx, Region::CAN_STAND_HERE);
                }
            }
        }
    }

    // Navigation elements
    for y in 0..REGION_HEIGHT {
        for x in 0..REGION_WIDTH {
            for z in 0..REGION_DEPTH {
                let idx = mapidx(x, y, z);
                if !region.flag(idx, Region::SOLID) {
                    if y > 0 && region.flag(idx - REGION_WIDTH, Region::CAN_STAND_HERE) {
                        region.set_flag(idx, Region::CAN_GO_NORTH)
                    }
                    if y < REGION_HEIGHT - 1
                        && region.flag(idx + REGION_WIDTH, Region::CAN_STAND_HERE)
                    {
                        region.set_flag(idx, Region::CAN_GO_SOUTH)
                    }
                    if x < REGION_WIDTH - 1 && region.flag(idx + 1, Region::CAN_STAND_HERE) {
                        region.set_flag(idx, Region::CAN_GO_EAST)
                    }
                    if x > 0 && region.flag(idx - 1, Region::CAN_STAND_HERE) {
                        region.set_flag(idx, Region::CAN_GO_WEST)
                    }

                    // TODO: Handle stairs and ramps properly
                    match region.tile_types[idx] {
                        TileType::Ramp { .. } => {
                            region.set_flag(idx, Region::CAN_GO_UP);
                            region.set_flag(
                                idx - (REGION_WIDTH * REGION_HEIGHT),
                                Region::CAN_GO_DOWN,
                            );
                        }
                        TileType::Stairs {
                            direction: StairsType::Up,
                        } => region.set_flag(idx, Region::CAN_GO_UP),
                        TileType::Stairs {
                            direction: StairsType::Down,
                        } => region.set_flag(idx, Region::CAN_GO_DOWN),
                        TileType::Stairs {
                            direction: StairsType::UpDown,
                        } => {
                            region.set_flag(idx, Region::CAN_GO_DOWN);
                            region.set_flag(idx, Region::CAN_GO_UP);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
