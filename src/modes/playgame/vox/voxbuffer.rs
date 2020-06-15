use crate::engine::VertexBuffer;
use crate::raws::*;
use std::collections::HashMap;
use super::greedy::*;
use super::modelsize::*;

pub struct VoxBuffer {
    pub vertices : VertexBuffer<f32>,
    pub offsets: Vec<(u32, u32)>
}

impl VoxBuffer {
    pub fn new() -> Self {
        Self {
            vertices: VertexBuffer::new(&[3, 1, 3]), // Position, normal index, tint
            offsets: Vec::new()
        }
    }

    pub fn load(&mut self, context: &crate::engine::Context) {
        self.vertices.clear();
        let rlock = RAWS.read();
        for modelfile in rlock.vox.vox.iter() {
            let filename = format!("resources/vox/{}.vox", modelfile.file);
            println!("Loading: {}", filename);
            let rawvox = dot_vox::load(&filename).unwrap();

            let mut last_index = 0;
            let mut cubes : HashMap<u32, u8> = HashMap::new();
            for model in rawvox.models.iter() {
                let size = ModelSize::new(model.size);
                for v in model.voxels.iter() {
                    let idx = size.vidx(v);
                    cubes.insert(idx, v.i);
                }
                greedy_cubes(&mut cubes, &mut self.vertices.data, &size);
                self.offsets.push((last_index, self.vertices.len()));
                last_index = self.vertices.len();
            }
        }

        self.vertices.build(&context.device, wgpu::BufferUsage::VERTEX);
    }
}