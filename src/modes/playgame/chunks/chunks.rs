use super::super::render_passes::frustrum::Frustrum;
use super::{Chunk, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_SIZE, CHUNK_WIDTH};
use crate::components::Position;
use crate::planet::Region;
use crate::modes::playgame::REGION;
use bracket_geometry::prelude::*;
use rayon::prelude::*;
use ultraviolet::Mat4;

pub struct Chunks {
    chunks: Vec<Chunk>,
    frustrum: Frustrum,
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

    pub fn rebuild_all(&mut self) {
        let rlock = REGION.read();
        self.chunks.par_iter_mut().for_each(|c| c.rebuild(&rlock));
    }

    pub fn on_camera_move(&mut self, camera_matrix: &Mat4, camera_position: &Position) {
        let cp = Point3::new(camera_position.x, camera_position.y, camera_position.z);
        self.frustrum.update(camera_matrix);
        self.visible_chunks = self
            .chunks
            .iter()
            .enumerate()
            .filter(|(_i, c)| {
                self.frustrum
                    .check_sphere(&c.center_pos, CHUNK_SIZE as f32 * 2.0)
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
