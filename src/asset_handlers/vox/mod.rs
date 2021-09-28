use std::collections::HashMap;

use bevy::{prelude::Mesh, render::mesh::VertexAttributeValues};
mod greedy;

pub fn load_vox_file(filename: &str) -> VoxTemplate {
    if let Ok(model) = dot_vox::load(filename) {
        println!("Loaded {}", filename);
        for model in &model.models {
            let mut template = VoxTemplate {
                width: model.size.x as u8,
                height: model.size.y as u8,
                depth: model.size.z as u8,
                voxels: HashMap::new(),
            };
            for vox in model.voxels.iter() {
                let idx = template.idx(vox.x, vox.y, vox.z);
                template.voxels.insert(idx, vox.i);
            }
            return template;
        }
        panic!("This shouldn't happen");
    } else {
        panic!("Failed to load {}", filename);
    }
}

pub struct VoxTemplate {
    pub width: u8,
    pub height: u8,
    pub depth: u8,
    pub voxels: HashMap<usize, u8>,
}

impl VoxTemplate {
    pub fn idx(&self, x: u8, y: u8, z: u8) -> usize {
        ((z as usize * self.width as usize * self.height as usize)
            + (y as usize * self.width as usize)
            + x as usize) as usize
    }

    pub fn idxmap(&self, mut idx: usize) -> (usize, usize, usize) {
        let layer_size: usize = self.width as usize * self.height as usize;
        let z = idx / layer_size;
        idx -= z * layer_size;

        let y = idx / self.width as usize;
        idx -= y * self.width as usize;

        let x = idx;
        (x, y, z)
    }

    pub fn merge(&mut self, other: &VoxTemplate) {
        if self.width != other.width || self.height != other.height || self.depth != other.depth {
            panic!("Cannot merge voxel templates of differing size");
        }

        for (idx, color) in other.voxels.iter() {
            self.voxels.insert(*idx, *color);
        }
    }

    pub fn to_mesh(&self) -> Mesh {
        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        greedy::greedy_cubes(self, &mut vertices, &mut normals, &mut uvs);
        let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
        mesh.set_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float3(vertices),
        );
        mesh.set_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            VertexAttributeValues::Float3(normals),
        );
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float2(uvs));
        mesh
    }
}
