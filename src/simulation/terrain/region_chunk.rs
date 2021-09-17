use super::{
    region_chunk_state::{ChunkState, ChunkStatus},
    GameCamera,
};
use crate::simulation::{
    planet_idx, terrain::chunker::Chunk, CHUNKS_PER_REGION, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH,
};
use bevy::{
    prelude::{Assets, Commands, Mesh, ResMut, Vec3},
    tasks::{AsyncComputeTaskPool, Task},
};

pub struct RegionChunk {
    pub required: bool,
    pub chunks: Vec<ChunkState>,
    pub tile_x: usize,
    pub tile_y: usize,
    pub chunk_builder_tasks: Vec<Task<ChunkBuilderTask>>,
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
            chunk_builder_tasks: Vec::new(),
        }
    }

    pub fn distance_activate(
        &mut self,
        camera: &GameCamera,
        mesh_assets: &mut ResMut<Assets<Mesh>>,
        commands: &mut Commands,
        task_master: AsyncComputeTaskPool,
    ) {
        let cam_pos = camera.look_at();
        let tx = self.tile_x;
        let ty = self.tile_y;
        self.chunks.iter_mut().enumerate().for_each(|(idx, c)| {
            let distance =
                Vec3::new(c.world_center.0, c.world_center.1, c.world_center.2).distance(cam_pos);
            //println!("{}", distance);
            if distance < 256.0 {
                // Ensure it's active
                //println!("Active chunk");
                c.activate(task_master.clone(), commands, tx, ty, idx);
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

    pub fn activate_entire_region(&mut self, task_master: AsyncComputeTaskPool) {
        let tx = self.tile_x;
        let ty = self.tile_y;
        let mut tasks = Vec::new();
        self.chunks.iter_mut().enumerate().for_each(|(idx, c)| {
            c.status = ChunkStatus::AsyncLoading;
            let base = c.base;
            let task = task_master.spawn(async move {
                let chunk = Chunk::generate(tx, ty, base.0, base.1, base.2);
                ChunkBuilderTask {
                    chunk,
                    planet_idx: planet_idx(tx, ty),
                    chunk_id: idx,
                }
            });
            tasks.push(task);
        });
        for t in tasks.drain(..) {
            self.chunk_builder_tasks.push(t);
        }
    }
}

pub struct ChunkBuilderTask {
    pub chunk: Chunk,
    pub planet_idx: usize,
    pub chunk_id: usize,
}
