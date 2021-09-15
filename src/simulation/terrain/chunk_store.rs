use crate::simulation::{CHUNKS_PER_REGION, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_SIZE, CHUNK_WIDTH, Planet, REGION_HEIGHT, REGION_WIDTH, WORLD_WIDTH, planet_idx};
use bevy::prelude::*;
use lazy_static::*;
use parking_lot::RwLock;
use std::collections::HashMap;

use super::{chunk_mesh::chunk_to_mesh, chunker::Chunk, strata::StrataMaterials, GameCamera};

lazy_static! {
    pub static ref CHUNK_STORE: RwLock<ChunkStore> = RwLock::new(ChunkStore::new());
}

pub struct ChunkStore {
    regions: HashMap<usize, RegionChunk>,
    strata: Option<StrataMaterials>,
    planet: Option<Planet>,
}

impl ChunkStore {
    pub fn new() -> Self {
        Self {
            regions: HashMap::new(),
            strata: None,
            planet: None,
        }
    }

    /// Call this once after raws have loaded
    pub fn verify_strata(&mut self) {
        self.strata = Some(StrataMaterials::read());
    }

    pub fn set_planet(&mut self, planet: Planet) {
        self.planet = Some(planet);
    }

    pub fn with_playable_region(&mut self, tile_x: usize, tile_y: usize) {
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
    }

    pub fn manage_for_camera(
        &mut self,
        camera: &GameCamera,
        mesh_assets: &mut ResMut<Assets<Mesh>>,
        world_material_handle: Handle<StandardMaterial>,
        commands: &mut Commands,
    ) {
        use std::collections::HashSet;
        let west = ((camera.tile_x * REGION_WIDTH) + camera.x - 128) / REGION_WIDTH;
        let east = ((camera.tile_x * REGION_WIDTH) + camera.x + 128) / REGION_WIDTH;
        let north = ((camera.tile_y * REGION_HEIGHT) + camera.y - 128) / REGION_HEIGHT;
        let south = ((camera.tile_y * REGION_HEIGHT) + camera.y + 128) / REGION_HEIGHT;

        let mut active_regions = HashSet::new();
        active_regions.insert(planet_idx(west, camera.tile_y));
        active_regions.insert(planet_idx(east, camera.tile_y));
        active_regions.insert(planet_idx(camera.tile_x, north));
        active_regions.insert(planet_idx(camera.tile_x, south));

        for pidx in active_regions.iter() {
            if let Some(r) = self.regions.get_mut(pidx) {
                //println!("Found active region: {}", pidx);
                r.distance_activate(
                    camera,
                    self.planet.as_ref().unwrap(),
                    self.strata.as_ref().unwrap(),
                    mesh_assets,
                    world_material_handle.clone(),
                    commands,
                );
            } else {
                //println!("Must activate new region: {}", pidx);
                let mut activate = RegionChunk::new(pidx % WORLD_WIDTH, pidx / WORLD_WIDTH);
                activate.distance_activate(
                    camera,
                    self.planet.as_ref().unwrap(),
                    self.strata.as_ref().unwrap(),
                    mesh_assets,
                    world_material_handle.clone(),
                    commands,
                );
                self.regions.insert(*pidx, activate);
            }
        }

        let mut to_destroy = HashSet::new();
        self.regions.iter().for_each(|(pidx, r)| {
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

pub fn chunk_index_in_region(x: usize, y: usize, z: usize) -> usize {
    (z * CHUNK_HEIGHT * CHUNK_WIDTH) + (y * CHUNK_WIDTH) + x
}

pub fn chunk_filename(tile_x: usize, tile_y: usize, x: usize, y: usize, z: usize) -> String {
    format!(
        "savegame/{}-{}.chunk",
        planet_idx(tile_x, tile_y),
        chunk_index_in_region(x, y, z)
    )
}

pub struct RegionChunk {
    pub required: bool,
    pub chunks: Vec<ChunkState>,
    pub tile_x: usize,
    pub tile_y: usize,
}

impl RegionChunk {
    pub fn new(tile_x: usize, tile_y: usize) -> Self {
        let mut chunks = Vec::with_capacity(CHUNKS_PER_REGION);
        for z in 0..CHUNK_DEPTH {
            for y in 0..CHUNK_HEIGHT {
                for x in 0..CHUNK_WIDTH {
                    chunks.push(ChunkState::new(tile_x, tile_y, x, y, z));
                }
            }
        }
        Self {
            chunks,
            required: false,
            tile_x,
            tile_y,
        }
    }

    pub fn distance_activate(
        &mut self,
        camera: &GameCamera,
        planet: &Planet,
        strata: &StrataMaterials,
        mesh_assets: &mut ResMut<Assets<Mesh>>,
        world_material_handle: Handle<StandardMaterial>,
        commands: &mut Commands,
    ) {
        let cam_pos = camera.pos_world();
        let tx = self.tile_x;
        let ty = self.tile_y;
        self.chunks.iter_mut().for_each(|c| {
            let distance = Vec3::new(c.world_center.0, c.world_center.1, c.world_center.2).distance(cam_pos);
            //println!("{}", distance);
            if distance < 256.0
            {
                // Ensure it's active
                //println!("Active chunk");
                c.activate(
                    planet,
                    strata,
                    mesh_assets,
                    world_material_handle.clone(),
                    commands,
                    tx,
                    ty,
                );
            } else {
                if !c.required {
                    // It's allowed to sleep now
                    //println!("Sleep chunk");
                    c.deactivate(mesh_assets);
                } else {
                    c.disable_render(mesh_assets);
                }
            }
        });
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum ChunkStatus {
    Expired,
    NotLoaded,
    Loaded,
}

#[derive(Clone)]
pub struct ChunkMesh(pub Handle<Mesh>);

#[derive(Clone)]
pub struct ChunkState {
    pub required: bool,
    pub dirty: bool,
    pub status: ChunkStatus,
    pub world_center: (f32, f32, f32),
    pub base: (usize, usize, usize),
    pub chunk: Option<Chunk>,
    pub mesh: Option<ChunkMesh>,
}

impl ChunkState {
    pub fn new(tile_x: usize, tile_y: usize, chunk_x: usize, chunk_y: usize, chunk_z: usize) -> Self {
        let cx = (tile_x as f32 * REGION_WIDTH as f32)
            + (chunk_x as f32 * CHUNK_SIZE as f32)
            + (CHUNK_WIDTH as f32 / 2.0);
        let cy = (tile_y as f32 * REGION_HEIGHT as f32)
            + (chunk_y as f32 * CHUNK_SIZE as f32)
            + (CHUNK_HEIGHT as f32 / 2.0);
        let cz = (chunk_z as f32 * CHUNK_SIZE as f32) + (CHUNK_SIZE as f32 / 2.0);
        Self {
            required: false,
            status: ChunkStatus::NotLoaded,
            dirty: false,
            world_center: (cx, cy, cz),
            chunk: None,
            mesh: None,
            base: (chunk_x * CHUNK_SIZE, chunk_y * CHUNK_SIZE, chunk_z * CHUNK_SIZE),
        }
    }

    pub fn deactivate(&mut self, mesh_assets: &mut ResMut<Assets<Mesh>>) {
        if let Some(mesh_handle) = &self.mesh {
            mesh_assets.remove(mesh_handle.0.clone());
        }
        self.chunk = None; // TODO: State management
        self.status = ChunkStatus::Expired;
    }

    pub fn disable_render(&mut self, mesh_assets: &mut ResMut<Assets<Mesh>>) {
        /*if let Some(mesh_handle) = &self.mesh {
            mesh_assets.remove(mesh_handle.0.clone());
        }*/
    }

    pub fn activate(
        &mut self,
        planet: &Planet,
        strata: &StrataMaterials,
        mesh_assets: &mut ResMut<Assets<Mesh>>,
        world_material_handle: Handle<StandardMaterial>,
        commands: &mut Commands,
        tile_x: usize,
        tile_y: usize,
    ) {
        if self.status != ChunkStatus::Loaded {
            // Load the chunk
            let region_x = self.base.0;
            let region_y = self.base.1;
            let region_z = self.base.2;
            self.chunk = Some(Chunk::generate(
                planet, strata, tile_x, tile_y, region_x, region_y, region_z,
            ));
            // Mesh it
            if let Some(mesh_handle) = &self.mesh {
                mesh_assets.remove(mesh_handle.0.clone());
            }
            let mesh = chunk_to_mesh(self.chunk.as_ref().unwrap());
            if mesh.is_some() {
                let asset_handle = mesh_assets.add(mesh.unwrap());
                self.mesh = Some(
                    ChunkMesh(asset_handle.clone())
                );
                let mx = (tile_x * REGION_WIDTH) as f32;
                let my = (tile_y * REGION_HEIGHT) as f32;
                let mz = 0.0;
                let mesh_entity = commands
                    .spawn_bundle(PbrBundle {
                        mesh: asset_handle.clone(),
                        material: world_material_handle.clone(),
                        transform: Transform::from_xyz(mx, my, mz),
                        ..Default::default()
                    });
            }
            self.status = ChunkStatus::Loaded;
        }
    }
}

// Next: chunk management service for loading/unloading
