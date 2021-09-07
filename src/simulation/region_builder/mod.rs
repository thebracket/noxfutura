use super::Planet;
use crate::simulation::{
    region_builder::{chunk_mesh::chunk_to_mesh, chunker::Chunk, strata::StrataMaterials},
    CHUNKS_PER_REGION, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_SIZE, CHUNK_WIDTH,
};
use bevy::prelude::Mesh;
use lazy_static::*;
use parking_lot::RwLock;
mod chunk_mesh;
mod chunker;
mod strata;
mod greedy;

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

    pub fn start(&mut self) {
        if !self.started {
            self.started = true;
            let p = self.planet.clone();
            let x = self.tile_x;
            let y = self.tile_y;
            std::thread::spawn(move || build_region(p, x, y));
        }
    }

    pub fn status(&self) -> String {
        match REGION_GEN.read().status {
            RegionBuilderStatus::Initializing => String::from("Initializing"),
            RegionBuilderStatus::MaterialMap => String::from("Reading the Map"),
            RegionBuilderStatus::Chunking => String::from("Dividing & Conquering"),
        }
    }

    pub fn chunks(&self) -> Option<Vec<Mesh>> {
        let is_some = REGION_GEN.read().chunks.is_some();
        if is_some {
            let mut writer = REGION_GEN.write();
            writer.chunks.take()
        } else {
            None
        }
    }
}

pub enum RegionBuilderStatus {
    Initializing,
    MaterialMap,
    Chunking,
}

pub struct RegionGen {
    pub status: RegionBuilderStatus,
    pub chunks: Option<Vec<Mesh>>,
}

impl RegionGen {
    pub fn new() -> Self {
        Self {
            status: RegionBuilderStatus::Initializing,
            chunks: None,
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
    println!("Reading material lists");
    update_status(RegionBuilderStatus::MaterialMap);
    let strata = StrataMaterials::read();

    println!("Chunking");
    update_status(RegionBuilderStatus::Chunking);
    let mut chunks = Vec::with_capacity(CHUNKS_PER_REGION);
    let mut meshes = Vec::new();
    for z in 0..CHUNK_DEPTH {
        let rz = z * CHUNK_DEPTH;
        for y in 0..CHUNK_HEIGHT {
            let ry = y * CHUNK_HEIGHT;
            for x in 0..CHUNK_WIDTH {
                let rx = x * CHUNK_WIDTH;
                chunks.push(Chunk::generate(
                    &planet, &strata, tile_x, tile_y, rx, ry, rz,
                ));
                if let Some(mesh) = chunk_to_mesh(&chunks[chunks.len() - 1]) {
                    meshes.push(mesh);
                }
            }
        }
    }
    println!("Made {} chunks, and {} meshes.", chunks.len(), meshes.len());
    {
        let mut w = REGION_GEN.write();
        w.chunks = Some(meshes);
    }

    println!("Done");
}
