use crate::planet::{ground_z, Region, TileType};
use bracket_geometry::prelude::Point;
use legion::*;
use crate::spatial::{mapidx, REGION_WIDTH};
use std::collections::HashSet;

pub fn debris_trail(region: &mut Region, ship_loc: Point, _ecs: &mut World) {
    let mut trees_to_remove = HashSet::new();

    for x in ship_loc.x - (REGION_WIDTH as i32 / 4)..ship_loc.x {
        for y in ship_loc.y - 3..ship_loc.y + 4 {
            let z = ground_z(region, x as usize, y as usize);

            let idx = mapidx(x as usize, y as usize, z);
            match region.tile_types[idx] {
                TileType::Floor { plant } => {
                    if plant.is_some() {
                        region.tile_types[idx] = TileType::Floor { plant: None };
                    }
                }
                TileType::TreeTrunk { tree_id } => {
                    trees_to_remove.insert(tree_id);
                }
                TileType::TreeFoliage { tree_id } => {
                    trees_to_remove.insert(tree_id);
                }
                _ => {}
            }
        }
    }

    region.tile_types.iter_mut().for_each(|t| match t {
        TileType::TreeTrunk { tree_id } => {
            if trees_to_remove.contains(&tree_id) {
                *t = TileType::Empty;
            }
        }
        TileType::TreeFoliage { tree_id } => {
            if trees_to_remove.contains(&tree_id) {
                *t = TileType::Empty;
            }
        }
        _ => {}
    });
}
