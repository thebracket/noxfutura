#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ModelSize {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl ModelSize {
    pub fn new(sz: dot_vox::Size) -> Self {
        Self {
            x: sz.x,
            y: sz.y,
            z: sz.z,
        }
    }

    pub fn idx(&self, x: u32, y: u32, z: u32) -> u32 {
        (z as u32 * self.x * self.y) + (y as u32 * self.x) + x as u32
    }

    pub fn idxmap(&self, mut idx: u32) -> (u32, u32, u32) {
        debug_assert!(idx < self.z * self.x * self.y);
        let layer_size: u32 = self.x as u32 * self.y as u32;
        let z = idx / layer_size;
        idx -= z * layer_size;

        let y = idx / self.x;
        idx -= y * self.x;

        let x = idx;
        debug_assert!(x <= self.x && y <= self.y && z < self.z);
        (x, y, z)
    }
}