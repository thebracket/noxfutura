use crate::components::*;
use crate::planet::{ground_z, Region};
use crate::spatial::REGION_WIDTH;
use bracket_geometry::prelude::Point;
pub use legion::systems::CommandBuffer;
use legion::*;
use std::collections::HashSet;

pub fn debris_trail(region: &mut Region, ship_loc: Point, ecs: &mut World) {
    let mut cb = CommandBuffer::new(ecs);
    let mut wood_to_spawn = HashSet::new();
    <(Entity, &Position)>::query()
        .filter(component::<Vegetation>() | component::<Tree>())
        .iter(ecs)
        .filter(|(_e, p)| {
            if let Some(p) = p.as_point3_only_tile() {
                if p.x >= ship_loc.x - (REGION_WIDTH as i32 / 4)
                    && p.x < ship_loc.x
                    && p.y >= ship_loc.y - 3
                    && p.y < ship_loc.y + 4
                    && p.z == ground_z(region, p.x as usize, p.y as usize) as i32
                {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        })
        .for_each(|(e, pos)| {
            if let Some(entry) = ecs.entry_ref(*e) {
                if let Ok(_tree) = entry.get_component::<Tree>() {
                    let pos = pos.as_point3();
                    wood_to_spawn.insert((pos.x, pos.y, pos.z));
                }
            }
            cb.remove(*e)
        });
    cb.flush(ecs);
    let wood = nox_raws::get_material_by_tag("Wood").unwrap();
    wood_to_spawn.iter().for_each(|(x, y, z)| {
        crate::planet::spawn_item_on_ground(
            ecs,
            "wood_log",
            *x as usize,
            *y as usize,
            *z as usize,
            region,
            wood,
        );
    });
}
