use super::{chunk_mesh::chunk_to_mesh, chunker::Chunk, PLANET_STORE};
use crate::simulation::{
    chunk_id, CHUNK_HEIGHT, CHUNK_SIZE, CHUNK_WIDTH, REGION_HEIGHT, REGION_WIDTH,
};
use bevy::prelude::{Assets, Commands, Handle, Mesh, PbrBundle, ResMut, Transform};

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
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: asset_handle.clone(),
                        material: PLANET_STORE
                            .read()
                            .world_material_handle
                            .as_ref()
                            .unwrap()
                            .clone(),
                        transform: Transform::from_xyz(mx, my, mz),
                        ..Default::default()
                    })
                    .insert(RenderChunk(chunk_id(
                        tile_x,
                        tile_y,
                        self.base.0,
                        self.base.1,
                        self.base.2,
                    )));
            }
            self.status = ChunkStatus::Loaded;
        }
    }
}

pub struct RenderChunk(pub usize);
