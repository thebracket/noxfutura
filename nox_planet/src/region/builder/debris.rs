use crate::{ground_z, Region};
use bracket_geometry::prelude::Point;
use legion::prelude::*;
use nox_components::*;
use nox_spatial::REGION_WIDTH;

pub fn debris_trail(region: &mut Region, ship_loc: Point, ecs: &mut World) {
    for x in ship_loc.x - (REGION_WIDTH as i32 / 4)..ship_loc.x {
        for y in ship_loc.y - 3..ship_loc.y + 4 {
            let z = ground_z(region, x as usize, y as usize);

            let veg_list_delete = <Read<Position>>::query()
                .filter(tag::<Vegetation>())
                .iter_entities_mut(ecs)
                .filter(|(_, pos)| pos.exact_position(x as usize, y as usize, z))
                .map(|(entity, _)| entity)
                .collect::<Vec<Entity>>();
            veg_list_delete.iter().for_each(|e| {
                ecs.delete(*e);
            });
        }
    }
}
