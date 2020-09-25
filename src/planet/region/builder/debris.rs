use crate::planet::{ground_z, Region};
use crate::spatial::REGION_WIDTH;
use bracket_geometry::prelude::Point;
use legion::*;
pub use legion::systems::CommandBuffer;
use crate::components::*;

pub fn debris_trail(region: &mut Region, ship_loc: Point, ecs: &mut World) {
    let mut cb = CommandBuffer::new(ecs);
    <(Entity, &Position)>::query()
        .filter(component::<Vegetation>() | component::<Tree>())
        .iter(ecs)
        .filter(|(_e, p)|
        {
            if let Some(p) = p.as_point3_only_tile() {
                if p.x >= ship_loc.x - (REGION_WIDTH as i32 / 4) &&
                    p.x < ship_loc.x &&
                    p.y >= ship_loc.y - 3 && p.y < ship_loc.y + 4 &&
                    p.z == ground_z(region, p.x as usize, p.y as usize) as i32
                {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }
        ).for_each(|(e, _)| cb.remove(*e));

    cb.flush(ecs);
}
