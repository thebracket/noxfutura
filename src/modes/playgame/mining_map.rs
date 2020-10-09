use crate::spatial::REGION_TILES_COUNT;
use super::systems::REGION;

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

    pub fn find_lowest_exit(&self, position: usize) -> Option<usize> {
        let rlock = REGION.read();
        let mut exits = rlock.get_available_exits(position);

        if exits.is_empty() {
            return None;
        }

        exits.sort_by(|a, b| {
            self.dijkstra[a.0 as usize]
                .partial_cmp(&self.dijkstra[b.0 as usize])
                .unwrap()
        });

        Some(exits[0].0)
    }
}