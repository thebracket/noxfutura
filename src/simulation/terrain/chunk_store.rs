use crate::simulation::{
    planet_idx, Planet, CHUNKS_PER_REGION, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_SIZE, CHUNK_WIDTH,
    REGION_HEIGHT, REGION_WIDTH, WORLD_WIDTH, chunk_id,
};
use bevy::prelude::*;
use bracket_noise::prelude::FastNoise;
use lazy_static::*;
use parking_lot::RwLock;
use std::collections::HashMap;

use super::{chunk_mesh::chunk_to_mesh, chunker::Chunk, strata::StrataMaterials, GameCamera};

lazy_static! {
    pub static ref CHUNK_STORE: RwLock<ChunkStore> = RwLock::new(ChunkStore::new());
}

lazy_static! {
    pub static ref PLANET_STORE: RwLock<PlanetData> = RwLock::new(PlanetData::new());
}

pub struct PlanetData {
    pub planet: Option<Planet>,
    pub strata: Option<StrataMaterials>,
    pub height_noise: Option<FastNoise>,
    pub material_noise: Option<FastNoise>,
    pub world_material_handle: Option<Handle<StandardMaterial>>,
}

impl PlanetData {
    pub fn new() -> Self {
        Self {
            planet: None,
            strata: None,
            height_noise: None,
            material_noise: None,
            world_material_handle: None,
        }
    }
}

pub struct ChunkStore {
    regions: HashMap<usize, RegionChunk>,
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
        commands: &mut Commands,
    ) {
        use std::collections::HashSet;

        let mut active_regions = HashSet::new();
        active_regions.insert(planet_idx(camera.tile_x, camera.tile_y));
        // Make this optional - load neighboring regions for context
        active_regions.insert(planet_idx(camera.tile_x - 1, camera.tile_y));
        active_regions.insert(planet_idx(camera.tile_x + 1, camera.tile_y));
        active_regions.insert(planet_idx(camera.tile_x, camera.tile_y + 1));
        active_regions.insert(planet_idx(camera.tile_x, camera.tile_y - 1));
        active_regions.insert(planet_idx(camera.tile_x -1, camera.tile_y - 1));
        active_regions.insert(planet_idx(camera.tile_x +1, camera.tile_y - 1));
        active_regions.insert(planet_idx(camera.tile_x-1, camera.tile_y + 1));
        active_regions.insert(planet_idx(camera.tile_x+1, camera.tile_y + 1));

        for pidx in active_regions.iter() {
            if let Some(r) = self.regions.get_mut(pidx) {
                //println!("Found active region: {}", pidx);
                r.distance_activate(
                    camera,
                    mesh_assets,
                    commands,
                );
            } else {
                //println!("Must activate new region: {}", pidx);
                let mut activate = RegionChunk::new(pidx % WORLD_WIDTH, pidx / WORLD_WIDTH);
                activate.distance_activate(
                    camera,
                    mesh_assets,
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
        mesh_assets: &mut ResMut<Assets<Mesh>>,
        commands: &mut Commands,
    ) {
        let cam_pos = camera.pos_world();
        let tx = self.tile_x;
        let ty = self.tile_y;
        self.chunks.iter_mut().for_each(|c| {
            let distance =
                Vec3::new(c.world_center.0, c.world_center.1, c.world_center.2).distance(cam_pos);
            //println!("{}", distance);
            if distance < 256.0 {
                // Ensure it's active
                //println!("Active chunk");
                c.activate(
                    mesh_assets,
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
    pub fn new(
        tile_x: usize,
        tile_y: usize,
        chunk_x: usize,
        chunk_y: usize,
        chunk_z: usize,
    ) -> Self {
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
            base: (
                chunk_x * CHUNK_SIZE,
                chunk_y * CHUNK_SIZE,
                chunk_z * CHUNK_SIZE,
            ),
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
        mesh_assets: &mut ResMut<Assets<Mesh>>,
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
                tile_x, tile_y, region_x, region_y, region_z,
            ));
            // Mesh it
            if let Some(mesh_handle) = &self.mesh {
                mesh_assets.remove(mesh_handle.0.clone());
            }
            let mesh = chunk_to_mesh(self.chunk.as_ref().unwrap());
            if mesh.is_some() {
                let asset_handle = mesh_assets.add(mesh.unwrap());
                self.mesh = Some(ChunkMesh(asset_handle.clone()));
                let mx = (tile_x * REGION_WIDTH) as f32;
                let my = (tile_y * REGION_HEIGHT) as f32;
                let mz = 0.0;
                commands.spawn_bundle(PbrBundle {
                    mesh: asset_handle.clone(),
                    material: PLANET_STORE.read().world_material_handle.as_ref().unwrap().clone(),
                    transform: Transform::from_xyz(mx, my, mz),
                    ..Default::default()
                })
                .insert(RenderChunk(chunk_id(tile_x, tile_y, self.base.0, self.base.1, self.base.2)));
            }
            self.status = ChunkStatus::Loaded;
        }
    }
}

pub struct RenderChunk(pub usize);