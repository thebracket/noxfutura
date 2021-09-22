use super::chunker::TileType;
use super::region_chunk::RegionChunk;
use super::region_chunk_state::ChunkStatus;
use super::PLANET_STORE;
use super::{strata::StrataMaterials, GameCamera};
use crate::simulation::{planet_idx, Planet, WORLD_WIDTH};
use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use lazy_static::*;
use parking_lot::RwLock;
use std::collections::HashMap;

lazy_static! {
    pub static ref CHUNK_STORE: RwLock<ChunkStore> = RwLock::new(ChunkStore::new());
}

pub struct ChunkStore {
    pub regions: HashMap<usize, RegionChunk>,
}

impl ChunkStore {
    pub fn new() -> Self {
        Self {
            regions: HashMap::new(),
        }
    }

    /// Call this once after raws have loaded
    pub fn verify_strata(&mut self) {
        PLANET_STORE.write().strata = Some(StrataMaterials::read());
    }

    /// Call this once a planet is loaded for the renderer to use.
    pub fn set_planet(&mut self, planet: Planet) {
        let height_noise = planet.get_height_noise();
        let mat_noise = planet.get_material_noise();
        PLANET_STORE.write().planet = Some(planet);
        PLANET_STORE.write().height_noise = Some(height_noise);
        PLANET_STORE.write().material_noise = Some(mat_noise);
    }

    /// Starts creating a playable region. The region will be marked to
    /// remain in memory - only swapping meshes in/out. Designed to run
    /// asynchronously, inside a write lock. It aims to bail-out fast,
    /// leaving the loading process running in the background. That integrates
    /// with the game camera, activating region chunks as they are ready.
    pub fn with_playable_region(
        &mut self,
        task_master: AsyncComputeTaskPool,
        tile_x: usize,
        tile_y: usize,
    ) {
        let region_idx = planet_idx(tile_x, tile_y);
        if let Some(region) = self.regions.get_mut(&region_idx) {
            // The region exists, just need to initialize it
            region.chunks.iter_mut().for_each(|c| {
                c.required = true;
                if c.status == ChunkStatus::Expired {
                    c.status = ChunkStatus::NotLoaded;
                }
            });
            region.required = true;
        } else {
            // New region
            let mut rc = RegionChunk::new(tile_x, tile_y);
            rc.required = true;
            rc.chunks.iter_mut().for_each(|c| c.required = true);
            self.regions.insert(region_idx, rc);
        }
        if let Some(region) = self.regions.get_mut(&region_idx) {
            region.activate_entire_region(task_master.clone());
        } else {
            panic!("Inserting the region failed.");
        }
    }

    pub fn is_region_fully_loaded(&self, tile_x: usize, tile_y: usize) -> bool {
        let idx = planet_idx(tile_x, tile_y);
        if let Some(region) = self.regions.get(&idx) {
            return region
                .chunks
                .iter()
                .filter(|c| c.status != ChunkStatus::Loaded)
                .count()
                == 0;
        } else {
            return false;
        }
    }

    pub fn manage_for_camera(
        &mut self,
        camera: &GameCamera,
        mesh_assets: &mut ResMut<Assets<Mesh>>,
        commands: &mut Commands,
        task_master: AsyncComputeTaskPool,
    ) {
        use std::collections::HashSet;

        let mut active_regions = HashSet::new();
        active_regions.insert(planet_idx(camera.tile_x, camera.tile_y));
        // Make this optional - load neighboring regions for context
        /*active_regions.insert(planet_idx(camera.tile_x - 1, camera.tile_y));
        active_regions.insert(planet_idx(camera.tile_x + 1, camera.tile_y));
        active_regions.insert(planet_idx(camera.tile_x, camera.tile_y + 1));
        active_regions.insert(planet_idx(camera.tile_x, camera.tile_y - 1));
        active_regions.insert(planet_idx(camera.tile_x - 1, camera.tile_y - 1));
        active_regions.insert(planet_idx(camera.tile_x + 1, camera.tile_y - 1));
        active_regions.insert(planet_idx(camera.tile_x - 1, camera.tile_y + 1));
        active_regions.insert(planet_idx(camera.tile_x + 1, camera.tile_y + 1));*/

        for pidx in active_regions.iter() {
            if let Some(r) = self.regions.get_mut(pidx) {
                //println!("Found active region: {}", pidx);
                r.distance_activate(camera, mesh_assets, commands, task_master.clone());
            } else {
                //println!("Must activate new region: {}", pidx);
                let mut activate = RegionChunk::new(pidx % WORLD_WIDTH, pidx / WORLD_WIDTH);
                activate.distance_activate(camera, mesh_assets, commands, task_master.clone());
                self.regions.insert(*pidx, activate);
            }
        }

        let mut to_destroy = HashSet::new();
        self.regions.iter_mut().for_each(|(pidx, r)| {
            if !r.required && !active_regions.contains(pidx) {
                //println!("Can deactivate {}", pidx);
                to_destroy.insert(*pidx);
            }
        });
        to_destroy.iter().for_each(|pidx| {
            self.regions.remove(pidx);
        });
    }
}

pub fn get_tile_type(region_idx: usize, tile_idx: usize) -> Option<TileType> {
    let rlock = CHUNK_STORE.read();
    if let Some(region) = rlock.regions.get(&region_idx) {
        region.get_tile_type(tile_idx)
    } else {
        None
    }
}

lazy_static! {
    pub static ref PLANET_CHANGE_QUEUE: RwLock<PlanetChangeQueue> = RwLock::new(PlanetChangeQueue::new());
}

pub enum TileChange{
    SetTileType{result: TileType},
}

pub struct PlanetChange{
    pub region_idx: usize,
    pub tile_idx: usize,
    pub change: TileChange,
}

pub struct PlanetChangeQueue{
    pub queue: Vec<PlanetChange>,
}

impl PlanetChangeQueue {
    pub fn new() -> Self {
        Self{
            queue: Vec::new(),
        }
    }
}

pub fn change_tile_type(region_idx: usize, tile_idx: usize, new_tile: TileType) {
    let mut queue = PLANET_CHANGE_QUEUE.write();
    queue.queue.push(
        PlanetChange{
            region_idx,
            tile_idx,
            change: TileChange::SetTileType{result: new_tile},
        }
    );
}

pub fn tile_changes_system(

) {
    if PLANET_CHANGE_QUEUE.read().queue.is_empty() {
        return;
    }

    let mut queue_lock = PLANET_CHANGE_QUEUE.write();
    for c in queue_lock.queue.drain(0..) {
        let mut rlock = CHUNK_STORE.write();
        let ridx = c.region_idx; // Copy so we're moving without borrow issues
        if let Some(region) = rlock.regions.get_mut(&ridx) {
            region.enqueue_change(c);
        }
    }
}