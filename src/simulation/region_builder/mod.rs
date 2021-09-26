use super::{
    terrain::{
        is_region_loaded, set_global_planet, spawn_playable_region, spawn_region_for_reference,
        PlanetLocation,
    },
    Planet,
};
use bevy::{prelude::Commands, tasks::AsyncComputeTaskPool};
use lazy_static::*;
use parking_lot::RwLock;
use std::time::Duration;
mod shipwright;
use crate::simulation::{
    mapidx,
    terrain::{
        are_regions_loaded, ground_z, submit_change_batch, ChangeRequest, MapChangeBatch,
        PLANET_STORE,
    },
    REGION_HEIGHT, REGION_WIDTH,
};

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
            RegionBuilderStatus::Water => String::from("Just Add Water"),
        }
    }
}

#[allow(dead_code)]
pub enum RegionBuilderStatus {
    Initializing,
    Chunking,
    Loaded,
    Water,
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

    // Load into memory for ramping - start the loading early
    let neighbors = [
        PlanetLocation::new(planet_idx.x - 1, planet_idx.y),
        PlanetLocation::new(planet_idx.x + 1, planet_idx.y),
        PlanetLocation::new(planet_idx.x, planet_idx.y - 1),
        PlanetLocation::new(planet_idx.x, planet_idx.y + 1),
    ];
    neighbors
        .iter()
        .for_each(|n| spawn_region_for_reference(*n));

    // Water features (temporary code)
    update_status(RegionBuilderStatus::Water);
    let water_level = PLANET_STORE.read().planet.as_ref().unwrap().water_height as usize;
    println!("Water level: {}", water_level);
    let mut changes = MapChangeBatch::new(planet_idx);
    for x in 0..REGION_WIDTH {
        for y in 0..REGION_HEIGHT {
            let ground = ground_z(planet_idx, x, y);
            if ground < water_level {
                for z in ground..water_level {
                    changes.enqueue_change(ChangeRequest::SolidTile {
                        idx: mapidx(x, y, z),
                        material: 0,
                    });
                }
            }
        }
    }
    submit_change_batch(changes);

    // Ramping
    update_status(RegionBuilderStatus::Ramping);
    while !are_regions_loaded(&neighbors) {
        std::thread::sleep(Duration::from_millis(10));
    }

    // Beaches

    // Vegetation

    // Crash the ship
    update_status(RegionBuilderStatus::Crashing);
    shipwright::build_escape_pod(planet_idx);

    // Trees

    // Blight

    // Debris Trail

    // Flags

    // Done
}
