use super::{
    mapidx,
    terrain::{is_region_loaded, set_global_planet, spawn_playable_region, PlanetLocation},
    Planet,
};
use bevy::{prelude::Commands, tasks::AsyncComputeTaskPool};
use lazy_static::*;
use parking_lot::RwLock;
use std::time::Duration;
mod shipwright;

pub struct RegionBuilder {
    planet: Planet,
    tile_x: usize,
    tile_y: usize,
    started: bool,
}

impl RegionBuilder {
    pub fn new(planet: Planet, tile_x: usize, tile_y: usize) -> Self {
        Self {
            planet,
            tile_x,
            tile_y,
            started: false,
        }
    }

    pub fn start(&mut self, task_master: AsyncComputeTaskPool, commands: &mut Commands) {
        if !self.started {
            self.started = true;
            let p = self.planet.clone();
            let x = self.tile_x;
            let y = self.tile_y;
            let task = task_master.spawn(async move {
                build_region(p, x, y);
            });
            commands.spawn().insert(task);
        }
    }

    pub fn status(&self) -> String {
        match REGION_GEN.read().status {
            RegionBuilderStatus::Initializing => String::from("Initializing"),
            RegionBuilderStatus::Chunking => String::from("Dividing & Conquering"),
            RegionBuilderStatus::Loaded => String::from("Region activated, making it pretty"),
            RegionBuilderStatus::Ramping => String::from("Smoothing Rough Edges"),
            RegionBuilderStatus::Crashing => String::from("Crash Landing"),
        }
    }
}

pub enum RegionBuilderStatus {
    Initializing,
    Chunking,
    Loaded,
    Ramping,
    Crashing,
}

pub struct RegionGen {
    pub status: RegionBuilderStatus,
}

impl RegionGen {
    pub fn new() -> Self {
        Self {
            status: RegionBuilderStatus::Initializing,
        }
    }
}

lazy_static! {
    static ref REGION_GEN: RwLock<RegionGen> = RwLock::new(RegionGen::new());
}

fn update_status(new_status: RegionBuilderStatus) {
    REGION_GEN.write().status = new_status;
}

fn build_region(planet: Planet, tile_x: usize, tile_y: usize) {
    set_global_planet(planet);
    update_status(RegionBuilderStatus::Chunking);
    let planet_idx = PlanetLocation::new(tile_x, tile_y);
    spawn_playable_region(planet_idx);
    while !is_region_loaded(planet_idx) {
        std::thread::sleep(Duration::from_millis(10));
    }
    update_status(RegionBuilderStatus::Loaded);

    // TODO: Ramp building

    // Crash the ship
    update_status(RegionBuilderStatus::Crashing);
    shipwright::build_escape_pod(planet_idx);
}
