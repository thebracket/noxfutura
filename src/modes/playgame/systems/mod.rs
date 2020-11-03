use bengine::random::RandomNumberGenerator;
use legion::*;
use nox_planet::Region;
use parking_lot::{Mutex, RwLock};
mod calendar;
mod camera_control;
mod construct_building;
mod end_turn;
mod initiative;
mod leisure_shift;
mod lumberjack;
mod mining;
mod mining_map;
mod move_randomly;
mod pause_control;
mod settler_scheduler;
mod sleep_shift;
pub mod toolfinder;
mod viewshed;
mod work_shift;

use super::messaging;

lazy_static! {
    pub static ref REGION: RwLock<Region> = RwLock::new(Region::initial());
}

lazy_static! {
    pub static ref RNG: Mutex<RandomNumberGenerator> = Mutex::new(RandomNumberGenerator::new());
}

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(mining_map::mining_map_system())
        .add_system(calendar::calendar_system())
        .add_system(viewshed::viewshed_system())
        .add_system(camera_control::camera_control_system())
        .add_system(pause_control::pause_control_system())
        .add_system(initiative::initiative_system())
        .flush()
        .add_system(settler_scheduler::settler_schedule_system())
        .flush()
        .add_system(leisure_shift::leisure_shift_system())
        .add_system(sleep_shift::sleep_shift_system())
        .add_system(work_shift::work_shift_system())
        .flush()
        .add_system(lumberjack::lumberjack_system())
        .add_system(construct_building::construction_system())
        .add_system(mining::mining_system())
        .add_system(move_randomly::move_randomly_system())
        .flush()
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn paused_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(viewshed::viewshed_system())
        .add_system(camera_control::camera_control_system())
        .add_system(pause_control::pause_control_system())
        .build()
}
