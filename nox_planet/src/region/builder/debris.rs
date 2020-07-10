use crate::{ground_z, Region, REGION_WIDTH};
use bracket_geometry::prelude::Point;
use legion::prelude::*;
use nox_components::*;

pub fn debris_trail(region: &mut Region, ship_loc: Point, ecs: &mut World) {
    for x in ship_loc.x - (REGION_WIDTH as i32 / 4)..ship_loc.x {
        for y in ship_loc.y - 3..ship_loc.y + 4 {
            let z = ground_z(region, x as usize, y as usize);

            let newtile_pt = Position {
                x: x as usize,
                y: y as usize,
                z: z as usize,
            };
            let veg_list_delete = <Read<Position>>::query()
                .filter(tag::<Vegetation>())
                .iter_entities_mut(ecs)
                .filter(|(_, pos)| **pos == newtile_pt)
                .map(|(entity, _)| entity)
                .collect::<Vec<Entity>>();
            veg_list_delete.iter().for_each(|e| {
                ecs.delete(*e);
            });
        }
    }
}
