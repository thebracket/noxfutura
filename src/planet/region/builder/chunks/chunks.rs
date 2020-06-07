use super::{Chunk, Primitive, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH};
use crate::planet::Region;
use rayon::prelude::*;

pub struct Chunks {
    chunks: Vec<Chunk>,
}

impl Chunks {
    pub fn empty() -> Self {
        let mut result = Self { chunks: Vec::new() };
        for z in 0..CHUNK_DEPTH {
            for y in 0..CHUNK_HEIGHT {
                for x in 0..CHUNK_WIDTH {
                    result.chunks.push(Chunk::new(x, y, z));
                }
            }
        }
        result
    }

    pub fn rebuild_all(&mut self, region: &Region, camera_z : i32) {
        self.chunks.par_iter_mut().for_each(|c| c.rebuild(region, camera_z));
    }

    pub fn all_geometry(&mut self) -> Vec<Primitive> {
        let mut result = Vec::new();
        self.chunks.iter_mut().for_each(|c| {
            if let Some(mut geometry) = c.geometry() {
                result.append(&mut geometry);
            }
        });
        result
    }
}
