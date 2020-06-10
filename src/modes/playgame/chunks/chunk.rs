use super::{chunk_idx, ChunkType, CHUNK_SIZE};
use crate::engine::VertexBuffer;
use crate::planet::{Region, TileType};
use crate::utils::{add_ramp_geometry, mapidx};
use std::collections::HashSet;
use ultraviolet::Vec3;

pub struct Chunk {
    pub t: ChunkType,
    pub idx: usize,
    pub base: (usize, usize, usize),
    cells: Vec<usize>,
    dirty: bool,
    vb: VertexBuffer<f32>,
    element_count: [u32; CHUNK_SIZE],
    pub center_pos: Vec3,
}

impl Chunk {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        let mut cells = Vec::with_capacity(CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE);
        for cz in z * CHUNK_SIZE..(z * CHUNK_SIZE) + CHUNK_SIZE {
            for cy in y * CHUNK_SIZE..(y * CHUNK_SIZE) + CHUNK_SIZE {
                for cx in x * CHUNK_SIZE..(x * CHUNK_SIZE) + CHUNK_SIZE {
                    cells.push(mapidx(cx, cy, cz));
                }
            }
        }

        Self {
            t: ChunkType::Empty,
            idx: chunk_idx(x, y, z),
            base: (x * CHUNK_SIZE, y * CHUNK_SIZE, z * CHUNK_SIZE),
            cells,
            dirty: true,
            vb: VertexBuffer::new(&[3, 2, 3]),
            element_count: [0; CHUNK_SIZE],
            center_pos: (
                (x * CHUNK_SIZE) as f32 + (CHUNK_SIZE / 2) as f32,
                (y * CHUNK_SIZE) as f32 + (CHUNK_SIZE / 2) as f32,
                (z * CHUNK_SIZE) as f32 + (CHUNK_SIZE / 2) as f32,
            )
                .into(),
        }
    }

    pub fn rebuild(&mut self, region: &Region, context: &crate::engine::Context) {
        if !self.dirty {
            return;
        }

        let mut count_empty = 0;
        let mut count_solid = 0;
        self.cells.iter().for_each(|idx| {
            if !region.revealed[*idx] {
                count_empty += 1;
            } else {
                match region.tile_types[*idx] {
                    TileType::Solid => count_solid += 1,
                    TileType::Empty => count_empty += 1,
                    _ => {}
                }
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
                self.vb.clear();
                self.element_count.iter_mut().for_each(|n| *n = 0);
            }
            ChunkType::Solid => {
                for z in 0..CHUNK_SIZE {
                    crate::utils::add_cube_geometry(
                        &mut self.vb.data,
                        &mut self.element_count[z],
                        self.base.0 as f32,
                        self.base.1 as f32,
                        self.base.2 as f32 + z as f32,
                        CHUNK_SIZE as f32,
                        CHUNK_SIZE as f32,
                        1.0,
                    );
                }
            }
            ChunkType::Partial => {
                for z in 0..CHUNK_SIZE {
                    let mut cubes = HashSet::new();
                    let mut floors = HashSet::new();
                    for y in 0..CHUNK_SIZE {
                        for x in 0..CHUNK_SIZE {
                            let idx = mapidx(x + self.base.0, y + self.base.1, z + self.base.2);
                            if region.revealed[idx] {
                                match region.tile_types[idx] {
                                    TileType::Solid => {
                                        cubes.insert(idx);
                                    }
                                    TileType::Floor => {
                                        floors.insert(idx);
                                    }
                                    TileType::Ramp { direction } => {
                                        add_ramp_geometry(
                                            &mut self.vb.data,
                                            &mut self.element_count[z],
                                            direction,
                                            x as f32 + self.base.0 as f32,
                                            y as f32 + self.base.1 as f32,
                                            z as f32 + self.base.2 as f32,
                                        );
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }

                    super::greedy::greedy_floors(
                        &mut floors,
                        &mut self.vb.data,
                        &mut self.element_count[z],
                    );
                    super::greedy::greedy_cubes(
                        &mut cubes,
                        &mut self.vb.data,
                        &mut self.element_count[z],
                    );
                }
            }
        }

        self.dirty = false;
        if self.vb.len() > 0 {
            self.vb.update_buffer(context);
        }
    }

    pub fn maybe_render_chunk(&self, camera_z: usize) -> Option<(&VertexBuffer<f32>, u32)> {
        if self.t == ChunkType::Empty {
            return None;
        }

        //let camera_ceiling = camera_z + 20;
        let mut n_elements = 0;
        for z in 0..CHUNK_SIZE {
            let layer_z = z + self.base.2;
            if layer_z <= camera_z {
                n_elements += self.element_count[z];
            }
        }

        if n_elements > 0 {
            Some((&self.vb, n_elements * 3))
        } else {
            None
        }
    }
}
