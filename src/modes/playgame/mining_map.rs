use crate::spatial::REGION_TILES_COUNT;

pub struct MiningMap {
    pub is_dirty: bool,
    pub dijkstra : Vec<f32>
}

impl MiningMap {
    pub fn new() -> Self {
        Self {
            is_dirty : true,
            dijkstra : vec![std::f32::MAX; REGION_TILES_COUNT]
        }
    }
}