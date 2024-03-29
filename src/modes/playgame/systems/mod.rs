use bengine::random::RandomNumberGenerator;
use legion::*;
use nox_planet::Region;
use parking_lot::{Mutex, RwLock};
mod automatic_reactions;
mod calendar;
mod camera_control;
mod component_hauling;
mod construct;
mod construct_building;
mod construction_designator;
mod construction_map;
mod end_turn;
mod initiative;
mod leisure_shift;
mod lumber_map;
mod lumberjack;
mod mining;
mod mining_map;
mod move_randomly;
mod pause_control;
mod reactions;
mod settler_scheduler;
mod sleep_shift;
mod tool_collection;
mod utils;
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
        .add_system(lumber_map::lumber_map_system())
        .add_system(construction_map::construction_map_system())
        .add_system(automatic_reactions::automatic_reactions_system())
        .add_system(calendar::calendar_system())
        .add_system(viewshed::viewshed_system())
        .add_system(camera_control::camera_control_system())
        .add_system(pause_control::pause_control_system())
        .add_system(initiative::initiative_system())
        .flush()
        .add_system(construction_designator::construction_designator_system())
        .add_system(settler_scheduler::settler_schedule_system())
        .flush()
        .add_system(leisure_shift::leisure_shift_system())
        .add_system(sleep_shift::sleep_shift_system())
        .add_system(work_shift::work_shift_system())
        .flush()
        .add_system(tool_collection::tool_collection_system())
        .add_system(component_hauling::hauling_system())
        .add_system(lumberjack::lumberjack_system())
        .add_system(construct::construction_system())
        .add_system(construct_building::construction_building_system())
        .add_system(mining::mining_system())
        .add_system(reactions::reactions_system())
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
