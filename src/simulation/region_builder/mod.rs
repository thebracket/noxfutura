use super::Planet;
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
        }
    }
}

pub enum RegionBuilderStatus {
    Initializing,
    Chunking,
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
    let mut cl = crate::simulation::terrain::CHUNK_STORE.write();
    cl.set_planet(planet);
    cl.with_playable_region(task_master.clone(), tile_x, tile_y);
    update_status(RegionBuilderStatus::Chunking);
    /*
    println!("Reading material lists");
    update_status(RegionBuilderStatus::MaterialMap);
    let strata = StrataMaterials::read();

    println!("Chunking");
    update_status(RegionBuilderStatus::Chunking);
    let mut chunks = Vec::with_capacity(CHUNKS_PER_REGION);
    for z in 0..CHUNK_DEPTH {
        let rz = z * CHUNK_DEPTH;
        for y in 0..CHUNK_HEIGHT {
            let ry = y * CHUNK_HEIGHT;
            for x in 0..CHUNK_WIDTH {
                let rx = x * CHUNK_WIDTH;
                chunks.push(Chunk::generate(
                    &planet, &strata, tile_x, tile_y, rx, ry, rz,
                ));
            }
        }
    }
    {
        let mut w = REGION_GEN.write();
        w.chunks = Some(chunks);
    }

    println!("Done"); */
}
