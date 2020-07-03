use crate::planet::{Region, TileType, REGION_TILES_COUNT, REGION_WIDTH};
use crate::utils::{ground_z, mapidx};
use bracket_geometry::prelude::Point;
use legion::prelude::*;

pub fn debris_trail(region: &mut Region, ship_loc: Point, ecs: &mut World) {
    for x in ship_loc.x - (REGION_WIDTH as i32 / 4)..ship_loc.x {
        for y in ship_loc.y - 3..ship_loc.y + 4 {
            let z = ground_z(region, x as usize, y as usize);
            let idx = mapidx(x as usize, y as usize, z as usize);
            region.vegetation_type_id[idx] = None;
            if region.tree_id[idx] > 0 {
                // Tree needs destruction
                let target_tree = region.tree_id[idx];
                let mut num_logs = 0;

                for tidx in 0..REGION_TILES_COUNT {
                    if region.tree_id[tidx] == target_tree {
                        region.tile_types[tidx] = TileType::Empty;
                        region.tree_id[tidx] = 0;
                        num_logs += 1;
                    }
                }
                region.tile_types[idx] = TileType::Floor;
                num_logs = (num_logs / 20) + 1;
                for _ in 0..num_logs {
                    nox_components::spawner::spawn_item_on_ground(
                        ecs, "wood_log", x as usize, y as usize, z as usize,
                    );
                }
            }
        }
    }
}
