use super::{Chunk, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_SIZE, CHUNK_WIDTH};
use crate::components::Position;
use crate::modes::playgame::systems::REGION;
use crate::utils::Frustrum;
use bracket_geometry::prelude::*;
use cgmath::Matrix4;
use rayon::prelude::*;

pub struct Chunks {
    chunks: Vec<Chunk>,
    pub frustrum: Frustrum,
    visible_chunks: Vec<(f32, usize)>,
}

impl Chunks {
    pub fn empty() -> Self {
        let mut result = Self {
            chunks: Vec::new(),
            frustrum: Frustrum::new(),
            visible_chunks: Vec::new(),
        };
        for z in 0..CHUNK_DEPTH {
            for y in 0..CHUNK_HEIGHT {
                for x in 0..CHUNK_WIDTH {
                    result.chunks.push(Chunk::new(x, y, z));
                }
            }
        }
        result
    }

    pub fn mark_dirty(&mut self, tiles: &Vec<usize>) {
        tiles.iter().for_each(|idx| {
            let (x, y, z) = crate::spatial::idxmap(*idx);
            let chunk_id = super::chunk_id_by_world(x, y, z);
            self.chunks[chunk_id].dirty = true;
        });
    }

    pub fn rebuild_all(&mut self) {
        let rlock = REGION.read();
        self.chunks.par_iter_mut().for_each(|c| c.rebuild(&rlock));
    }

    pub fn on_camera_move(&mut self, camera_matrix: &Matrix4<f32>, camera_position: &Position) {
        let cp = camera_position.as_point3();
        self.frustrum.update(camera_matrix);
        self.visible_chunks = self
            .chunks
            .iter()
            .enumerate()
            .filter(|(_i, c)| {
                self.frustrum
                    .check_sphere(&c.center_pos, CHUNK_SIZE as f32 / 1.5)
            })
            .map(|(i, c)| {
                (
                    DistanceAlg::PythagorasSquared.distance3d(
                        cp,
                        Point3::new(
                            c.center_pos.x as i32,
                            c.center_pos.y as i32,
                            c.center_pos.z as i32,
                        ),
                    ),
                    i,
                )
            })
            .collect();
        // Sort with nearest first to encourage z-buffer removal
        self.visible_chunks
            .sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    pub fn visible_chunks(&self) -> Vec<&Chunk> {
        self.visible_chunks
            .iter()
            .map(|(_, i)| &self.chunks[*i])
            .collect()
    }
}
