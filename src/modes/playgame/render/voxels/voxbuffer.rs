use super::greedy::*;
use super::modelsize::*;
use nox_raws::*;
use bengine::*;
use std::collections::HashMap;

pub struct VoxBuffer {
    pub vertices: FloatBuffer<f32>,
    pub offsets: Vec<(u32, u32)>,
}

impl VoxBuffer {
    pub fn new() -> Self {
        Self {
            vertices: FloatBuffer::new(&[3, 1, 1], 100, gpu::BufferUsage::VERTEX), // Position, normal index, tint
            offsets: Vec::new(),
        }
    }

    pub fn load(&mut self, palette: &Palette) {
        self.vertices.clear();
        let rlock = RAWS.read();
        let mut last_index = 0;
        for modelfile in rlock.vox.vox.iter() {
            let filename = format!("resources/vox/{}.vox", modelfile.file);
            let rawvox = dot_vox::load(&filename).unwrap();

            let mut cubes: HashMap<i32, u8> = HashMap::new();
            for model in rawvox.models.iter() {
                let size = ModelSize::new(model.size);
                for v in model.voxels.iter() {
                    let idx = size.vidx(v) as i32;
                    cubes.insert(idx, v.i);
                }
                greedy_cubes(&mut cubes, &mut self.vertices.data, &size, palette);
                assert_ne!(last_index, self.vertices.len());
                self.offsets.push((last_index, self.vertices.len()));
                last_index = self.vertices.len();
            }
        }

        self.vertices.build();
    }
}
