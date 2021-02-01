use super::greedy::*;
use super::{chunk_idx, ChunkType, CHUNK_SIZE};
use bengine::uv::Vec3;
use bengine::*;
use nox_planet::{Region, TileType};
use nox_spatial::mapidx;
use nox_utils::add_ramp_geometry;

pub struct Chunk {
    pub t: ChunkType,
    pub idx: usize,
    pub base: (usize, usize, usize),
    cells: Vec<usize>,
    pub dirty: bool,
    floors: FloatBuffer<f32>,
    vb: FloatBuffer<f32>,
    design_buffer: FloatBuffer<f32>,
    element_count: [u32; CHUNK_SIZE],
    floor_element_count: [u32; CHUNK_SIZE],
    design_element_count: [u32; CHUNK_SIZE],
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
            vb: FloatBuffer::new(
                &[3, 2, 1, 1, 1],
                10,
                gpu::BufferUsage::VERTEX | gpu::BufferUsage::COPY_DST,
            ),
            floors: FloatBuffer::new(
                &[3, 2, 1, 1, 1],
                10,
                gpu::BufferUsage::VERTEX | gpu::BufferUsage::COPY_DST,
            ),
            design_buffer: FloatBuffer::new(
                &[3, 2, 1, 1, 1],
                10,
                gpu::BufferUsage::VERTEX | gpu::BufferUsage::COPY_DST,
            ),
            element_count: [0; CHUNK_SIZE],
            floor_element_count: [0; CHUNK_SIZE],
            design_element_count: [0; CHUNK_SIZE],
            center_pos: (
                (x * CHUNK_SIZE) as f32 + (CHUNK_SIZE / 2) as f32,
                (y * CHUNK_SIZE) as f32 + (CHUNK_SIZE / 2) as f32,
                (z * CHUNK_SIZE) as f32 + (CHUNK_SIZE / 2) as f32,
            )
                .into(),
        }
    }

    #[inline]
    fn calc_material(&self, idx: usize, region: &Region) -> (usize, bool) {
        (
            region.material_idx[idx],
            region.flag(idx, Region::CONSTRUCTED),
        )
    }

    #[inline]
    fn calc_floor_material(&self, idx: usize, region: &Region) -> (usize, bool) {
        (
            region.material_idx[idx],
            region.flag(idx, Region::CONSTRUCTED),
        )
    }

    #[inline]
    fn water_material(&self) -> (usize, bool) {
        (nox_raws::get_material_by_tag("Water").unwrap(), false)
    }

    #[inline]
    fn hidden_material(&self) -> (usize, bool) {
        (nox_raws::get_material_by_tag("Hidden").unwrap(), false)
    }

    pub fn rebuild(&mut self, region: &Region) {
        if !self.dirty {
            return;
        }
        self.dirty = false;
        self.vb.clear();
        self.floors.clear();
        self.design_buffer.clear();
        self.element_count = [0; CHUNK_SIZE];
        self.floor_element_count = [0; CHUNK_SIZE];
        self.design_element_count = [0; CHUNK_SIZE];

        let mut count_empty = 0;
        self.cells
            .iter()
            .for_each(|idx| match region.tile_types[*idx] {
                TileType::Empty => count_empty += 1,
                _ => {}
            });

        let len = self.cells.len();

        if count_empty == len {
            self.t = ChunkType::Empty;
        } else {
            self.t = ChunkType::Partial;
        }

        for z in 0..CHUNK_SIZE {
            let mut cubes = CubeMap::new();
            let mut floors = CubeMap::new();
            let mut dcubes = CubeMap::new();
            let mut dfloors = CubeMap::new();
            for y in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let idx = mapidx(x + self.base.0, y + self.base.1, z + self.base.2);
                    if region.revealed[idx] {
                        match region.tile_types[idx] {
                            TileType::Solid => {
                                cubes.insert(idx, self.calc_material(idx, region));
                                dcubes.insert(idx, self.calc_material(idx, region));
                            }
                            TileType::Floor { .. } => {
                                floors.insert(idx, self.calc_floor_material(idx, region));
                                dfloors.insert(idx, self.calc_floor_material(idx, region));
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
                                add_ramp_geometry(
                                    &mut self.design_buffer.data,
                                    &mut self.design_element_count[z],
                                    direction,
                                    x as f32 + self.base.0 as f32,
                                    y as f32 + self.base.1 as f32,
                                    z as f32 + self.base.2 as f32,
                                    mat,
                                );
                            }
                            _ => {
                                dfloors.insert(idx, self.hidden_material());
                            }
                        }
                        // Temporary water 2
                        if region.water_level[idx] > 0 {
                            floors.insert(idx, self.water_material());
                            dfloors.insert(idx, self.water_material());
                        }
                    } else {
                        floors.insert(idx, self.hidden_material());
                        dfloors.insert(idx, self.hidden_material());
                    }
                }
            }

            super::greedy::greedy_floors(
                &mut floors,
                &mut self.floors.data,
                &mut self.floor_element_count[z],
            );
            super::greedy::greedy_cubes(
                &mut cubes,
                &mut self.vb.data,
                &mut self.element_count[z],
            );
            super::greedy::greedy_floors(&mut dfloors, &mut self.design_buffer.data, &mut self.design_element_count[z]);
            super::greedy::greedy_cubes(&mut dcubes, &mut self.design_buffer.data, &mut self.design_element_count[z]);
        }

        self.dirty = false;
        if self.vb.len() > 0 {
            //println!("Updated buffer");
            self.vb.update_buffer();
        }
        if self.floors.len() > 0 {
            self.floors.update_buffer();
        }
        if self.design_buffer.len() > 0 {
            self.design_buffer.update_buffer();
        }
    }

    pub fn maybe_render_chunk(&self, camera_z: i32) -> Option<(&FloatBuffer<f32>, u32)> {
        if self.t == ChunkType::Empty {
            return None;
        }

        //let camera_ceiling = camera_z + 20;
        let mut n_elements = 0;
        for z in 0..CHUNK_SIZE {
            let layer_z = z + self.base.2;
            if layer_z <= camera_z as usize {
                n_elements += self.element_count[z];
            }
        }

        if n_elements > 0 {
            Some((&self.vb, n_elements * 3))
        } else {
            None
        }
    }

    pub fn maybe_render_chunk_floors(&self, camera_z: i32) -> Option<(&FloatBuffer<f32>, u32)> {
        if self.t == ChunkType::Empty {
            return None;
        }

        //let camera_ceiling = camera_z + 20;
        let mut n_elements = 0;
        for z in 0..CHUNK_SIZE {
            let layer_z = z + self.base.2;
            if layer_z <= camera_z as usize {
                n_elements += self.floor_element_count[z];
            }
        }

        if n_elements > 0 {
            Some((&self.floors, n_elements * 3))
        } else {
            None
        }
    }

    pub fn maybe_render_chunk_design(&self, camera_z: i32) -> Option<(&FloatBuffer<f32>, u32, u32)> {
        if self.t == ChunkType::Empty {
            return None;
        }

        let mut start = 0;
        let mut end = 0;
        for z in 0..CHUNK_SIZE {
            let layer_z = z + self.base.2;
            if layer_z < camera_z as usize {
                start += self.design_element_count[z];
                end += self.design_element_count[z];
            } else if layer_z == camera_z as usize {
                end += self.design_element_count[z];
            }
        }

        if end > 0 {
            Some((&self.design_buffer, start * 3, end * 3))
        } else {
            None
        }
    }
}
