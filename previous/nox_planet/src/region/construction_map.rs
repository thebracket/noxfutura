use crate::Region;
use nox_spatial::REGION_TILES_COUNT;

pub struct ConstructionMap {
    pub is_dirty: bool,
    pub dijkstra: Vec<f32>,
}

impl ConstructionMap {
    pub fn new() -> Self {
        Self {
            is_dirty: true,
            dijkstra: vec![std::f32::MAX; REGION_TILES_COUNT],
        }
    }

    pub fn find_lowest_exit(&self, position: usize, region: &Region) -> Option<usize> {
        let mut exits = region.get_available_exits(position);

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
