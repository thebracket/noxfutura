use super::{chunk_idx, ChunkType, Primitive, CHUNK_SIZE};
use crate::planet::{Region, TileType};
use crate::utils::mapidx;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Chunk {
    pub t: ChunkType,
    pub idx: usize,
    pub base: (usize, usize, usize),
    geometry: Option<Vec<Primitive>>,
}

impl Chunk {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self {
            t: ChunkType::Empty,
            idx: chunk_idx(x, y, z),
            base: (x * CHUNK_SIZE, y * CHUNK_SIZE, z * CHUNK_SIZE),
            geometry: None,
        }
    }

    pub fn iter(&self) -> ChunkIter {
        ChunkIter {
            base: self.base,
            current: (0, 0, 0),
            done: false,
        }
    }

    pub fn rebuild(&mut self, region: &Region) {
        let len = self.iter().count();
        let mut count_empty = 0;
        let mut count_solid = 0;
        self.iter().for_each(|idx| {
            let idx = mapidx(idx.0, idx.1, idx.2);
            match region.tile_types[idx] {
                TileType::Solid => count_solid += 1,
                TileType::Empty => count_empty += 1,
                _ => {}
            }
        });
        if count_empty == len {
            self.t = ChunkType::Empty;
        } else if count_solid == len {
            self.t = ChunkType::Solid;
        } else {
            self.t = ChunkType::Partial;
        }

        self.geometry = self.build_geometry(region);
    }

    pub fn geometry(&mut self) -> Option<Vec<Primitive>> {
        self.geometry.clone()
    }

    fn build_geometry(&self, region: &Region) -> Option<Vec<Primitive>> {
        match self.t {
            ChunkType::Solid => Some(vec![
                Primitive::Cube {
                    x: self.base.0,
                    y: self.base.1,
                    z: self.base.2,
                    w: CHUNK_SIZE,
                    d: CHUNK_SIZE,
                    h: CHUNK_SIZE
                };
                1
            ]),
            ChunkType::Partial  => {
                let mut p = Vec::new();
                let mut cubes = HashSet::new();
                self.iter().for_each(|pos| {
                    let idx = mapidx(pos.0, pos.1, pos.2);
                    match region.tile_types[idx] {
                        TileType::Solid => {
                            //println!("{},{},{} = {}", pos.0, pos.1, pos.2, idx);
                            cubes.insert(idx);
                        }
                        _ => {}
                    }
                });
                p.append(&mut super::greedy::greedy_cubes(cubes));
                Some(p)
            }
            _ => None,
        }
    }
}

pub struct ChunkIter {
    base: (usize, usize, usize),
    current: (usize, usize, usize),
    done: bool,
}

impl Iterator for ChunkIter {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<(usize, usize, usize)> {
        if self.done {
            return None;
        }
        let result = (
            self.current.0 + self.base.0,
            self.current.1 + self.base.1,
            self.current.2 + self.base.2,
        );

        self.current.0 += 1;
        if self.current.0 == CHUNK_SIZE {
            self.current.0 = 0;

            self.current.1 += 1;
            if self.current.1 == CHUNK_SIZE {
                self.current.1 = 0;

                self.current.2 += 1;
                if self.current.2 == CHUNK_SIZE {
                    self.done = true;
                }
            }
        }

        Some(result)
    }
}
