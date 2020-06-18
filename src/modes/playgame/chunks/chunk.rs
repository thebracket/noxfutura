use super::super::ChunkModel;
use super::greedy::*;
use super::{chunk_idx, ChunkType, CHUNK_SIZE};
use crate::engine::{VertexBuffer, DEVICE_CONTEXT};
use crate::planet::{Region, StairsType, TileType};
use crate::utils::{add_floor_geometry, add_ramp_geometry, mapidx};
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
    pub chunk_models: Vec<ChunkModel>,
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
            chunk_models: Vec::new(),
        }
    }

    pub fn rebuild(&mut self, region: &Region) {
        if !self.dirty {
            return;
        }
        self.chunk_models.clear();

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
                                        let mat = crate::raws::RAWS
                                            .read()
                                            .matmap
                                            .get(region.material_idx[idx])
                                            .texture;
                                        cubes.insert(idx, mat);
                                    }
                                    TileType::TreeTrunk => {
                                        // bark
                                        let mat = crate::raws::RAWS.read().matmap.bark_id;
                                        cubes.insert(idx, mat);
                                    }
                                    TileType::TreeFoliage => {
                                        // leaf
                                        let mat = crate::raws::RAWS.read().matmap.leaf_id;
                                        cubes.insert(idx, mat);
                                    }
                                    TileType::Floor => {
                                        let mat = if let Some(_plant_idx) =
                                            region.vegetation_type_id[idx]
                                        {
                                            crate::raws::RAWS.read().matmap.grass_id
                                        } else {
                                            crate::raws::RAWS
                                                .read()
                                                .matmap
                                                .get(region.material_idx[idx])
                                                .floor
                                        };
                                        floors.insert(idx, mat);
                                    }
                                    TileType::Ramp { direction } => {
                                        let mat = crate::raws::RAWS
                                            .read()
                                            .matmap
                                            .get(region.material_idx[idx])
                                            .texture;
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
                                    TileType::Stairs { direction } => {
                                        let tag = match direction {
                                            StairsType::Up => "stairs_up",
                                            StairsType::Down => "stairs_down",
                                            StairsType::UpDown => "stairs_updown",
                                        };
                                        self.chunk_models.push(ChunkModel {
                                            id: crate::raws::RAWS.read().vox.get_model_idx(tag),
                                            x: x + self.base.0,
                                            y: y + self.base.1,
                                            z: z + self.base.2,
                                        });
                                    }
                                    _ => {}
                                }

                                // Add water - temporarily here, it'll have to move
                                let wl = region.water_level[idx];
                                if wl > 0 {
                                    let mat = crate::raws::RAWS.read().matmap.water_id;
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
            self.vb.update_buffer();
        }
    }

    pub fn maybe_render_chunk(
        &self,
        camera_z: usize,
        render_chunks: &mut Vec<ChunkModel>,
    ) -> Option<(&VertexBuffer<f32>, u32)> {
        if self.t == ChunkType::Empty {
            return None;
        }

        render_chunks.extend_from_slice(&self.chunk_models);

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
