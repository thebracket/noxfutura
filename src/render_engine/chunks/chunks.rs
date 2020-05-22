use super::{CHUNK_WIDTH, CHUNK_HEIGHT, CHUNK_DEPTH, Chunk, Primitive};
use crate::region::Region;

pub struct Chunks {
    chunks : Vec<Chunk>
}

impl Chunks {
    pub fn empty() -> Self {
        let mut result = Self {
            chunks: Vec::new()
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

    pub fn rebuild_all(&mut self, region : &Region) {
        self.chunks.iter_mut().for_each(|c| c.rebuild(region));
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
