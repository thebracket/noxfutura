use super::super::{WORLD_HEIGHT, WORLD_WIDTH};

pub struct WaterParticle {
    pub idx: usize,
    pub history: Vec<usize>,
    pub done: bool,
    pub deposits: bool,
}

impl WaterParticle {
    pub fn new(idx: usize) -> Self {
        let mut history = Vec::new();
        history.push(idx);
        Self {
            idx,
            history,
            done: false,
            deposits: true,
        }
    }

    fn candidate(&self, base_map: &[i16], candidates: &mut Vec<(usize, i16)>, candidate: usize) {
        if !self.history.contains(&candidate) {
            candidates.push((candidate, base_map[candidate]));
        }
    }

    pub fn flow(&mut self, base_map: &[i16]) {
        let mut candidates = Vec::with_capacity(8);
        let x = self.idx % WORLD_WIDTH;
        let y = self.idx / WORLD_HEIGHT;

        if x > 0 && x < WORLD_WIDTH - 1 && y > 0 && y < WORLD_HEIGHT - 1 {
            // Cardinals
            self.candidate(base_map, &mut candidates, self.idx - 1);
            self.candidate(base_map, &mut candidates, self.idx + 1);
            self.candidate(base_map, &mut candidates, self.idx - WORLD_WIDTH);
            self.candidate(base_map, &mut candidates, self.idx + WORLD_WIDTH);

            // Diagonals
            self.candidate(base_map, &mut candidates, (self.idx - WORLD_WIDTH) - 1);
            self.candidate(base_map, &mut candidates, (self.idx - WORLD_WIDTH) + 1);
            self.candidate(base_map, &mut candidates, (self.idx + WORLD_WIDTH) - 1);
            self.candidate(base_map, &mut candidates, (self.idx + WORLD_WIDTH) + 1);

            if !candidates.is_empty() {
                candidates.sort_by(|a, b| a.1.cmp(&b.1));
                self.idx = candidates[0].0;
                self.history.push(self.idx);
                if base_map[self.idx] < 1 {
                    self.done = true;
                }
            } else {
                self.done = true;
                self.deposits = false;
            }
        } else {
            self.done = true;
            self.deposits = false;
        }
    }
}
