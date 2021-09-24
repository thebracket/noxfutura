use crate::simulation::mapidx;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChunkLocation {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl ChunkLocation {
    pub fn new(x:usize, y: usize, z: usize) -> Self {
        Self {x,y,z}
    }

    pub fn to_tile_index(&self) -> usize {
        mapidx(self.x, self.y, self.z)
    }
}