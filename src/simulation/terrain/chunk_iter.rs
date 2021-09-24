use super::ChunkLocation;
use crate::simulation::CHUNK_SIZE;

pub struct ChunkIterator {
    chunk_base: ChunkLocation,
    current: ChunkLocation,
    done: bool,
}

impl ChunkIterator {
    pub fn new(chunk_base: ChunkLocation) -> Self {
        Self {
            chunk_base,
            current: chunk_base,
            done: false,
        }
    }
}

impl Iterator for ChunkIterator {
    type Item = ChunkLocation;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result = self.current;
        self.current.x += 1;
        if self.current.x == self.chunk_base.x + CHUNK_SIZE {
            self.current.x = self.chunk_base.x;
            self.current.y += 1;
            if self.current.y == self.chunk_base.y + CHUNK_SIZE {
                self.current.y = self.chunk_base.y;
                self.current.z += 1;
                if self.current.z == self.chunk_base.z + CHUNK_SIZE {
                    self.done = true;
                }
            }
        }
        Some(result)
    }
}

impl ExactSizeIterator for ChunkIterator {
    fn len(&self) -> usize {
        CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE
    }
}

#[cfg(test)]
mod test {
    use crate::simulation::REGION_TILES_COUNT;

    use super::*;

    #[test]
    fn test_length() {
        let mut n = 0;
        for _ in ChunkIterator::new(ChunkLocation { x: 0, y: 0, z: 0 }) {
            n += 1;
        }
        assert_eq!(CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE, n);
    }

    #[test]
    fn test_bounds() {
        for n in ChunkIterator::new(ChunkLocation { x: 0, y: 0, z: 0 }) {
            assert!(n.x < CHUNK_SIZE);
            assert!(n.y < CHUNK_SIZE);
            assert!(n.z < CHUNK_SIZE);
        }
    }

    #[test]
    fn test_bounds_full() {
        use crate::simulation::{CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH};
        let mut count = 0;
        for z in 0..CHUNK_DEPTH {
            for y in 0..CHUNK_HEIGHT {
                for x in 0..CHUNK_WIDTH {
                    let base = ChunkLocation {
                        x: x * CHUNK_SIZE,
                        y: y * CHUNK_SIZE,
                        z: z * CHUNK_SIZE,
                    };
                    for n in ChunkIterator::new(base) {
                        assert!(n.x >= base.x);
                        assert!(n.x < base.x + CHUNK_SIZE);
                        assert!(n.y >= base.y);
                        assert!(n.y < base.y + CHUNK_SIZE);
                        assert!(n.z >= base.z);
                        assert!(n.z < base.z + CHUNK_SIZE);
                        count += 1;
                    }
                }
            }
        }
        assert_eq!(REGION_TILES_COUNT, count);
    }
}
