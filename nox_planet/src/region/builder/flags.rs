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
                match region.tile_types[idx] {
                    TileType::Solid
                    | TileType::SemiMoltenRock
                    | TileType::Wall
                    | TileType::Window => blocked = true,
                    _ => {}
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
                        TileType::Floor { .. } | TileType::Stairs { .. } => {
                            can_stand = true;
                        }
                        TileType::Ramp { .. } => {
                            can_stand = true;
                            let up = mapidx(x, y, z + 1);
                            region.set_flag(up, Region::CAN_STAND_HERE);
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
                if region.flag(idx, Region::CAN_STAND_HERE) {
                    if valid_exit(region, x - 1, y, z) {
                        region.set_flag(idx, Region::CAN_GO_WEST)
                    }
                    if valid_exit(region, x + 1, y, z) {
                        region.set_flag(idx, Region::CAN_GO_EAST)
                    }
                    if valid_exit(region, x, y - 1, z) {
                        region.set_flag(idx, Region::CAN_GO_NORTH)
                    }
                    if valid_exit(region, x, y + 1, z) {
                        region.set_flag(idx, Region::CAN_GO_SOUTH)
                    }

                    match region.tile_types[idx] {
                        TileType::Ramp { .. } => {
                            let up = mapidx(x, y, z + 1);
                            region.set_flag(idx, Region::CAN_GO_UP);
                            region.set_flag(up, Region::CAN_GO_DOWN);
                            region.set_flag(idx, Region::CAN_STAND_HERE);
                        }

                        TileType::Stairs {
                            direction: StairsType::Up,
                        } => {
                            if valid_exit(region, x, y, z + 1) {
                                region.set_flag(idx, Region::CAN_GO_UP)
                            }
                        }
                        TileType::Stairs {
                            direction: StairsType::Down,
                        } => {
                            if valid_exit(region, x, y, z - 1) {
                                region.set_flag(idx, Region::CAN_GO_DOWN)
                            }
                        }
                        TileType::Stairs {
                            direction: StairsType::UpDown,
                        } => {
                            if valid_exit(region, x, y, z + 1) {
                                region.set_flag(idx, Region::CAN_GO_UP)
                            }
                            if valid_exit(region, x, y, z - 1) {
                                region.set_flag(idx, Region::CAN_GO_DOWN)
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn valid_exit(region: &Region, x: usize, y: usize, z: usize) -> bool {
    if x > 0 && x < REGION_WIDTH && y > 0 && y < REGION_HEIGHT && z > 0 && z < REGION_DEPTH {
        let idx = mapidx(x, y, z);
        region.flag(idx, Region::CAN_STAND_HERE)
    } else {
        false
    }
}

/*pub fn localized_flags(region: &mut Region, idx: usize) {
    let (x, y, z) = idxmap(idx);

    region.clear_flag(idx, Region::CAN_GO_DOWN);
    region.clear_flag(idx, Region::CAN_GO_UP);
    region.clear_flag(idx, Region::CAN_GO_NORTH);
    region.clear_flag(idx, Region::CAN_GO_SOUTH);
    region.clear_flag(idx, Region::CAN_GO_EAST);
    region.clear_flag(idx, Region::CAN_GO_WEST);
    region.clear_flag(idx, Region::CAN_STAND_HERE);
    region.clear_flag(idx, Region::SOLID);
    region.clear_flag(idx, Region::OUTSIDE);
    region.clear_flag(idx, Region::CONSTRUCTED);

    // Solidity
    match region.tile_types[idx] {
        TileType::SemiMoltenRock => region.set_flag(idx, Region::SOLID),
        TileType::Solid => region.set_flag(idx, Region::SOLID),
        TileType::Wall => region.set_flag(idx, Region::SOLID),
        TileType::Window => region.set_flag(idx, Region::SOLID),
        _ => {}
    }

    // Outdoor lighting
    let mut blocked = false;
    for z in (0..REGION_DEPTH).rev() {
        let idx = mapidx(x, y, z);
        if !blocked {
            region.set_flag(idx, Region::OUTSIDE);
        }
        match region.tile_types[idx] {
            TileType::Solid | TileType::SemiMoltenRock | TileType::Wall | TileType::Window => {
                blocked = true
            }
            _ => {}
        }
    }

    // Standing
    let mut can_stand = false;
    if !region.flag(idx, Region::SOLID) {
        match region.tile_types[idx] {
            TileType::Floor { .. } | TileType::Stairs { .. } => {
                can_stand = true;
            }
            TileType::Ramp { .. } => {
                can_stand = true;
                let up = mapidx(x, y, z + 1);
                region.set_flag(up, Region::CAN_STAND_HERE);
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

    // Navigation
    if region.flag(idx, Region::CAN_STAND_HERE) {
        if valid_exit(region, x - 1, y, z) {
            region.set_flag(idx, Region::CAN_GO_WEST)
        }
        if valid_exit(region, x + 1, y, z) {
            region.set_flag(idx, Region::CAN_GO_EAST)
        }
        if valid_exit(region, x, y - 1, z) {
            region.set_flag(idx, Region::CAN_GO_NORTH)
        }
        if valid_exit(region, x - 1, y + 1, z) {
            region.set_flag(idx, Region::CAN_GO_SOUTH)
        }

        match region.tile_types[idx] {
            TileType::Ramp { .. } => {
                let up = mapidx(x, y, z + 1);
                region.set_flag(idx, Region::CAN_GO_UP);
                region.set_flag(up, Region::CAN_GO_DOWN);
                region.set_flag(idx, Region::CAN_STAND_HERE);
            }

            TileType::Stairs {
                direction: StairsType::Up,
            } => {
                if valid_exit(region, x, y, z + 1) {
                    region.set_flag(idx, Region::CAN_GO_UP)
                }
            }
            TileType::Stairs {
                direction: StairsType::Down,
            } => {
                if valid_exit(region, x, y, z - 1) {
                    region.set_flag(idx, Region::CAN_GO_DOWN)
                }
            }
            TileType::Stairs {
                direction: StairsType::UpDown,
            } => {
                if valid_exit(region, x, y, z + 1) {
                    region.set_flag(idx, Region::CAN_GO_UP)
                }
                if valid_exit(region, x, y, z - 1) {
                    region.set_flag(idx, Region::CAN_GO_DOWN)
                }
            }
            _ => {}
        }
    }
}
*/
