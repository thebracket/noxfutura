use std::time::Duration;

use super::{Planet, mapidx, terrain::{change_tile_type, chunker::{RampDirection, TileType}}};
use crate::simulation::{REGION_HEIGHT, REGION_WIDTH, planet_idx, terrain::{CHUNK_STORE, chunker::cell_altitude, get_tile_type}};
use bevy::{prelude::Commands, tasks::AsyncComputeTaskPool};
use lazy_static::*;
use parking_lot::RwLock;

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
            let tm = task_master.clone();
            let task = task_master.spawn(async move {
                build_region(p, x, y, tm);
            });
            commands.spawn().insert(task);
        }
    }

    pub fn status(&self) -> String {
        match REGION_GEN.read().status {
            RegionBuilderStatus::Initializing => String::from("Initializing"),
            RegionBuilderStatus::Chunking => String::from("Dividing & Conquering"),
            RegionBuilderStatus::Loaded => String::from("Region activated, making it pretty"),
            RegionBuilderStatus::Ramping => String::from("Ramping up the volume"),
        }
    }
}

pub enum RegionBuilderStatus {
    Initializing,
    Chunking,
    Loaded,
    Ramping,
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

fn build_region(planet: Planet, tile_x: usize, tile_y: usize, task_master: AsyncComputeTaskPool) {
    // First block: starts the building process
    {
        let mut cl = CHUNK_STORE.write();
        cl.set_planet(planet);
        cl.with_playable_region(task_master.clone(), tile_x, tile_y);
        update_status(RegionBuilderStatus::Chunking);
    }

    // We need to wait until the region is in memory
    while !CHUNK_STORE.read().is_region_fully_loaded(tile_x, tile_y) {
        std::thread::sleep(Duration::from_micros(10));
    }
    update_status(RegionBuilderStatus::Loaded);

    let mut altitudes = vec![0; REGION_WIDTH * REGION_HEIGHT];
    {
        use crate::simulation::terrain::PLANET_STORE;
        let plock = PLANET_STORE.read();
        let noise = plock.height_noise.as_ref().unwrap();
        for y in 0..REGION_HEIGHT {
            for x in 0..REGION_WIDTH {
                let altitude = cell_altitude(&noise, tile_x, tile_y, x, y);
                let altitude_idx = (y * REGION_WIDTH) + x;
                altitudes[altitude_idx] = altitude;
            }
        }
    }

    // Time to make ramps
    update_status(RegionBuilderStatus::Ramping);
    ramping(planet_idx(tile_x, tile_y), &altitudes);
}

fn is_floor(planet_idx: usize, x: usize, y: usize, z: usize) -> bool {
    match get_tile_type(planet_idx, mapidx(x, y, z-1)) {
        Some(TileType::Solid{..}) => true,
        _ => false,
    }
}

fn get_material(planet_idx: usize, x: usize, y: usize, z: usize) -> usize {
    match get_tile_type(planet_idx, mapidx(x, y, z)) {
        Some(TileType::Solid{ material}) => material,
        Some(TileType::Ramp{ material, ..}) => material,
        _ => 0,
    }
}

fn ramping(planet_idx: usize, altitudes: &[u32]) {
    for y in 1..REGION_HEIGHT-1 {
        for x in 1..REGION_WIDTH-1 {
            let z = altitudes[(y * REGION_WIDTH)+x] as usize;
            if is_floor(planet_idx, x, y, z) {
                let material = get_material(planet_idx, x, y, z-1);
                if is_floor(planet_idx, x, y-1, z) {
                    change_tile_type(planet_idx, mapidx(x, y, z), TileType::Ramp{direction: RampDirection::NorthSouth, material});
                }
                else if is_floor(planet_idx, x, y+1, z) {
                    change_tile_type(planet_idx, mapidx(x, y, z), TileType::Ramp{direction: RampDirection::SouthNorth, material});
                }
                else if is_floor(planet_idx, x+1, y, z) {
                    change_tile_type(planet_idx, mapidx(x, y, z), TileType::Ramp{direction: RampDirection::WestEast, material});
                }
                else if is_floor(planet_idx, x-1, y-1, z) {
                    change_tile_type(planet_idx, mapidx(x, y, z), TileType::Ramp{direction: RampDirection::EastWest, material});
                }
            }
        }
    }
}