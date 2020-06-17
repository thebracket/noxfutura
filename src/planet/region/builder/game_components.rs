use crate::components::*;
use crate::planet::REGION_WIDTH;
use bracket_geometry::prelude::Point;
use legion::prelude::*;

pub fn add_game_components(world: &mut World, hm: &[u8], crash_site: Point) {
    world.insert(
        (Cordex {},),
        (0..1).map(|_| {
            (
                Identity::new(),
                Position {
                    x: 128,
                    y: 128,
                    z: hm[(128 * REGION_WIDTH) + 128] as _,
                },
                CameraOptions {
                    zoom_level: 10,
                    mode: CameraMode::TopDown,
                },
                WorldPosition {
                    planet_x: crash_site.x as _,
                    planet_y: crash_site.y as _,
                },
                Calendar {
                    year: 2525,
                    month: 0,
                    day: 0,
                    hour: 0,
                    minute: 0,
                    second: 0,
                },
            )
        }),
    );
}
