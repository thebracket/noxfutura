use super::{region_chunk_state::ChunkState, GameCamera};
use crate::simulation::{CHUNKS_PER_REGION, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH};
use bevy::prelude::{Assets, Commands, Mesh, ResMut, Vec3};

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
                c.activate(mesh_assets, commands, tx, ty);
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
