use crate::components::*;
use nox_spatial::{REGION_WIDTH, WORLD_WIDTH};
use bengine::geometry::Point;
use legion::*;

pub fn add_game_components(world: &mut World, hm: &[u8], crash_site: Point) {
    world.push((
        Cordex {},
        IdentityTag::new(),
        Position::with_tile(
            128,
            128,
            hm[(128 * REGION_WIDTH) + 128] as usize,
            ((crash_site.y * WORLD_WIDTH as i32) + crash_site.x) as usize,
            (1, 1, 1),
        ),
        CameraOptions {
            zoom_level: 10,
            mode: CameraMode::TopDown,
        },
        Calendar {
            year: 2525,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
        },
    ));
}
