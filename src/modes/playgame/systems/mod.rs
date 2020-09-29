use crate::planet::Region;
use bracket_random::prelude::RandomNumberGenerator;
use legion::*;
use parking_lot::{Mutex, RwLock};
mod camera_control;
mod viewshed;
mod calendar;

lazy_static! {
    pub static ref REGION: RwLock<Region> = RwLock::new(Region::initial());
}

lazy_static! {
    pub static ref RNG: Mutex<RandomNumberGenerator> = Mutex::new(RandomNumberGenerator::new());
}

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(calendar::calendar_system())
        .add_system(viewshed::viewshed_system())
        .add_system(camera_control::camera_control_system())
        .build()
}

pub fn paused_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(viewshed::viewshed_system())
        .add_system(camera_control::camera_control_system())
        .build()
}
