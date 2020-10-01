use crate::spatial::{REGION_DEPTH, REGION_HEIGHT, REGION_WIDTH};

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_WIDTH: usize = REGION_WIDTH as usize / CHUNK_SIZE;
pub const CHUNK_HEIGHT: usize = REGION_HEIGHT as usize / CHUNK_SIZE;
pub const CHUNK_DEPTH: usize = REGION_DEPTH as usize / CHUNK_SIZE;
//pub const CHUNK_TOTAL: usize = CHUNK_HEIGHT * CHUNK_WIDTH;
//pub const CHUNKS_TOTAL: usize = CHUNK_WIDTH * CHUNK_HEIGHT * CHUNK_DEPTH;

pub fn chunk_idx(x: usize, y: usize, z: usize) -> usize {
    (z * CHUNK_HEIGHT * CHUNK_WIDTH) + (y * CHUNK_WIDTH) + x
}

pub fn chunk_id_by_world(x: usize, y: usize, z: usize) -> usize {
    chunk_idx(x / CHUNK_SIZE, y / CHUNK_SIZE, z / CHUNK_SIZE)
}
