use super::{Chunk, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH, CHUNK_SIZE, super::frustrum::Frustrum};
use crate::planet::Region;
use rayon::prelude::*;
use ultraviolet::Mat4;

pub struct Chunks {
    chunks: Vec<Chunk>,
}

impl Chunks {
    pub fn empty() -> Self {
        let mut result = Self { 
            chunks: Vec::new(),
        };
        for z in 0..CHUNK_DEPTH {
            for y in 0..CHUNK_HEIGHT {
                for x in 0..CHUNK_WIDTH {
                    result.chunks.push(Chunk::new(x, y, z));
                }
            }
        }
        result
    }

    pub fn rebuild_all(&mut self, region: &Region, context: &crate::engine::Context) {
        self.chunks.par_iter_mut().for_each(|c| c.rebuild(region, context));
    }

    pub fn visible_chunks(&self, camera_matrix : &Mat4) -> Vec<&Chunk> {
        let mut frustrum = Frustrum::new();
        frustrum.update(camera_matrix);
        self.chunks
            .iter()
            .filter(|c| {
                frustrum.check_sphere(
                    &c.center_pos,
                    CHUNK_SIZE as f32 * 2.0
                )
            })
            .collect()
    }
}
