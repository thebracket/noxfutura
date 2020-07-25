use super::super::ChunkModel;
use super::greedy::*;
use super::{chunk_idx, ChunkType, CHUNK_SIZE};
use crate::engine::VertexBuffer;
use crate::utils::add_floor_geometry;
use nox_planet::{RampDirection, Region, StairsType, TileType};
use nox_raws::{MappedTexture, RAWS};
use nox_spatial::mapidx;
use cgmath::Vector3;
use crate::utils::add_ramp_geometry;

pub struct Chunk {
    pub t: ChunkType,
    pub idx: usize,
    pub base: (usize, usize, usize),
    cells: Vec<usize>,
    pub dirty: bool,
    vb: VertexBuffer<f32>,
    element_count: [u32; CHUNK_SIZE],
    pub center_pos: Vector3<f32>,
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
            vb: VertexBuffer::new(&[3, 1, 2, 1]),
            element_count: [0; CHUNK_SIZE],
            center_pos: (
                (x * CHUNK_SIZE) as f32 + (CHUNK_SIZE / 2) as f32,
                (y * CHUNK_SIZE) as f32 + (CHUNK_SIZE / 2) as f32,
                (z * CHUNK_SIZE) as f32 + (CHUNK_SIZE / 2) as f32,
            )
                .into(),
        }
    }

    fn calc_material(&self, idx: usize, region: &Region) -> MappedTexture {
        if region.flag(idx, Region::CONSTRUCTED) {
            RAWS.read().matmap.get(region.material_idx[idx]).constructed
        } else {
            RAWS.read().matmap.get(region.material_idx[idx]).texture
        }
    }

    fn calc_floor_material(&self, idx: usize, region: &Region) -> MappedTexture {
        if region.flag(idx, Region::CONSTRUCTED) {
            RAWS.read()
                .matmap
                .get(region.material_idx[idx])
                .floor_constructed
        } else {
            RAWS.read().matmap.get(region.material_idx[idx]).floor
        }
    }

    pub fn rebuild(&mut self, region: &Region) {
        if !self.dirty {
            return;
        }
        self.dirty = false;
        self.vb.clear();
        self.element_count = [0, 0, 0, 0, 0, 0, 0, 0];

        let mut count_empty = 0;
        self.cells.iter().for_each(|idx| {
            if !region.revealed[*idx] {
                count_empty += 1;
            } else {
                match region.tile_types[*idx] {
                    TileType::Empty => count_empty += 1,
                    _ => {}
                }
            }
        });

        let len = self.cells.len();

        if count_empty == len {
            self.t = ChunkType::Empty;
        } else {
            self.t = ChunkType::Partial;
        }

        match self.t {
            ChunkType::Empty => {
                self.vb.clear();
                self.element_count.iter_mut().for_each(|n| *n = 0);
            }
            ChunkType::Partial => {
                for z in 0..CHUNK_SIZE {
                    let mut cubes = CubeMap::new();
                    let mut floors = CubeMap::new();
                    for y in 0..CHUNK_SIZE {
                        for x in 0..CHUNK_SIZE {
                            let idx = mapidx(x + self.base.0, y + self.base.1, z + self.base.2);
                            if region.revealed[idx] {
                                match region.tile_types[idx] {
                                    TileType::Solid => {
                                        cubes.insert(idx, self.calc_material(idx, region));
                                    }
                                    TileType::TreeTrunk{..} => {
                                        cubes.insert(idx, MappedTexture{ texture: RAWS.read().matmap.bark_id, tint: (1.0, 1.0, 1.0) });
                                    }
                                    TileType::TreeFoliage{..} => {
                                        cubes.insert(idx, MappedTexture{ texture: RAWS.read().matmap.leaf_id, tint: (1.0, 1.0, 1.0) });
                                    }
                                    TileType::Floor{plant} => {
                                        if let Some(_plant) = plant {
                                            floors.insert(idx, MappedTexture{ texture: RAWS.read().matmap.grass_id, tint: (1.0, 1.0, 1.0) });
                                        } else {
                                            floors.insert(idx, self.calc_floor_material(idx, region));
                                        }
                                    }
                                    TileType::Ramp { direction } => {
                                        let mat = self.calc_floor_material(idx, region);
                                        add_ramp_geometry(
                                            &mut self.vb.data,
                                            &mut self.element_count[z],
                                            direction,
                                            x as f32 + self.base.0 as f32,
                                            y as f32 + self.base.1 as f32,
                                            z as f32 + self.base.2 as f32,
                                            mat,
                                        );
                                    }
                                    _ => {}
                                }

                                // Add water - temporarily here, it'll have to move
                                let wl = region.water_level[idx];
                                if wl > 0 {
                                    let mat = MappedTexture {
                                        texture: RAWS.read().matmap.water_id,
                                        tint: (1.0, 1.0, 1.0),
                                    };
                                    add_floor_geometry(
                                        &mut self.vb.data,
                                        &mut self.element_count[z],
                                        x as f32 + self.base.0 as f32,
                                        y as f32 + self.base.1 as f32,
                                        z as f32 + self.base.2 as f32 + (wl as f32 / 10.0),
                                        1.0,
                                        1.0,
                                        mat,
                                    )
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
            //println!("Updated buffer");
            self.vb.update_buffer();
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
