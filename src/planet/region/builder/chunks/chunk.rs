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
    cells : Vec<usize>,
    dirty : bool,
    layers : Vec<Vec<f32>>
}

impl Chunk {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        let mut cells = Vec::with_capacity(CHUNK_SIZE*CHUNK_SIZE*CHUNK_SIZE);
        for cz in z * CHUNK_SIZE .. (z * CHUNK_SIZE) + CHUNK_SIZE {
            for cy in y * CHUNK_SIZE .. (y * CHUNK_SIZE) + CHUNK_SIZE {
                for cx in x * CHUNK_SIZE .. (x * CHUNK_SIZE) + CHUNK_SIZE {
                    cells.push(mapidx(cx, cy, cz));
                }
            }
        }

        Self {
            t: ChunkType::Empty,
            idx: chunk_idx(x, y, z),
            base: (x * CHUNK_SIZE, y * CHUNK_SIZE, z * CHUNK_SIZE),
            geometry: None,
            cells,
            dirty : true,
            layers : vec![Vec::new(); CHUNK_SIZE]
        }
    }

    pub fn rebuild(&mut self, region: &Region) {

        if !self.dirty {
            return;
        }

        let mut count_empty = 0;
        let mut count_solid = 0;
        self.cells.iter().for_each(|idx| {
            match region.tile_types[*idx] {
                TileType::Solid => count_solid += 1,
                TileType::Empty => count_empty += 1,
                _ => {}
            }
        });

        let len = self.cells.len();

        if count_empty == len {
            self.t = ChunkType::Empty;
        } else if count_solid == len {
            self.t = ChunkType::Solid;
        } else {
            self.t = ChunkType::Partial;
        }

        match self.t {
            ChunkType::Empty => {
                self.layers.iter_mut().for_each(|v| v.clear());
            }
            ChunkType::Solid => {
                for z in 0..CHUNK_SIZE {
                    crate::utils::add_cube_geometry(
                        &mut self.layers[z],
                        self.base.0 as f32,
                        self.base.1 as f32,
                        self.base.2 as f32 + z as f32,
                        CHUNK_SIZE as f32,
                        CHUNK_SIZE as f32,
                        1.0
                    );
                }
            }
            ChunkType::Partial => {
                for z in 0..CHUNK_SIZE {
                    let mut cubes = HashSet::new();
                    for y in 0..CHUNK_SIZE {
                        for x in 0..CHUNK_SIZE {
                            let idx = mapidx(x + self.base.0, y + self.base.1, z + self.base.2);
                            match region.tile_types[idx] {
                                TileType::Solid => {
                                    //println!("{},{},{} = {}", pos.0, pos.1, pos.2, idx);
                                    cubes.insert(idx);
                                }
                                _ => {}
                            }
                        }
                    }

                    super::greedy::greedy_cubes(&mut cubes, &mut self.layers[z]);
                }
            }
            _ => {}
        }

        self.dirty = false;
    }

    pub fn append_geometry(&self, camera_z : usize, slice: &mut Vec<f32>) {
        /*if self.t == ChunkType::Empty {
            return;
        }
        if camera_z < self.base.2 + CHUNK_SIZE {
            return;
        }*/

        for z in 0..CHUNK_SIZE {
            let lz = z + self.base.2;
            if lz <= camera_z && lz > camera_z - 10 {
                slice.extend_from_slice(&self.layers[z]);
            }
        }
    }
}
