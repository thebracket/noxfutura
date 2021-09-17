use super::{
    chunk_mesh::chunk_to_mesh, chunker::Chunk, region_chunk::ChunkBuilderTask, MeshBuilderTask,
};
use crate::simulation::{
    planet_idx, CHUNK_HEIGHT, CHUNK_SIZE, CHUNK_WIDTH, REGION_HEIGHT, REGION_WIDTH,
};
use bevy::{
    prelude::{Assets, Commands, Handle, Mesh, ResMut},
    tasks::AsyncComputeTaskPool,
};

#[derive(Clone, PartialEq, Eq)]
pub enum ChunkStatus {
    Expired,
    NotLoaded,
    AsyncLoading,
    LoadedNoMesh,
    AsyncMeshing,
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
        if self.mesh.is_some() {
            if let Some(mesh_handle) = &self.mesh {
                mesh_assets.remove(mesh_handle.0.clone());
            }
        }
        self.mesh = None;
        self.chunk = None;
        self.status = ChunkStatus::Expired;
    }

    pub fn disable_render(&mut self, mesh_assets: &mut ResMut<Assets<Mesh>>) {
        if self.mesh.is_some() {
            if let Some(mesh_handle) = &self.mesh {
                mesh_assets.remove(mesh_handle.0.clone());
            }
            self.mesh = None;
            self.status = ChunkStatus::LoadedNoMesh;
        }
    }

    pub fn activate(
        &mut self,
        task_master: AsyncComputeTaskPool,
        commands: &mut Commands,
        tile_x: usize,
        tile_y: usize,
        idx: usize,
    ) {
        match self.status {
            ChunkStatus::NotLoaded | ChunkStatus::Expired => {
                self.start_loading_chunk(task_master, commands, tile_x, tile_y, idx)
            }
            ChunkStatus::LoadedNoMesh => {
                self.regenerate_mesh(task_master, commands, planet_idx(tile_x, tile_y), idx)
            }
            _ => {}
        }
    }

    fn start_loading_chunk(
        &mut self,
        task_master: AsyncComputeTaskPool,
        commands: &mut Commands,
        tile_x: usize,
        tile_y: usize,
        idx: usize,
    ) {
        self.status = ChunkStatus::AsyncLoading;
        let base = self.base;
        let task = task_master.spawn(async move {
            let chunk = Chunk::generate(tile_x, tile_y, base.0, base.1, base.2);
            ChunkBuilderTask {
                chunk,
                planet_idx: planet_idx(tile_x, tile_y),
                chunk_id: idx,
            }
        });
        commands.spawn().insert(task);
    }

    fn regenerate_mesh(
        &mut self,
        task_master: AsyncComputeTaskPool,
        commands: &mut Commands,
        planet_idx: usize,
        chunk_id: usize,
    ) {
        let chunk_clone = self.chunk.as_ref().unwrap().clone();
        let task = task_master.spawn(async move {
            let mesh = chunk_to_mesh(&chunk_clone);
            MeshBuilderTask {
                mesh,
                planet_idx: planet_idx,
                chunk_id: chunk_id,
            }
        });
        commands.spawn().insert(task);
    }
}

pub struct RenderChunk(pub usize);
