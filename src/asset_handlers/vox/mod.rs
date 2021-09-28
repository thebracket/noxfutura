use std::{collections::HashMap, path::Path};
use bevy::{prelude::Mesh, render::mesh::VertexAttributeValues};
use crate::asset_handlers::vox::greedy::VoxMap;
use self::model_size::ModelSize;
mod model_size;
mod greedy;

pub fn load_vox_file(filename: &str) -> VoxTemplate {
    if let Ok(model) = dot_vox::load(filename) {
        println!("Loaded {}", filename);
        for model in &model.models {
            let mut template = VoxTemplate {
                size: ModelSize::new(model.size),
                voxels: VoxMap::new(),
            };
            for vox in model.voxels.iter() {
                let idx = template.size.idx(vox.x.into(), vox.y.into(), vox.z.into());
                template.voxels.insert(idx as i32, vox.i);
            }
            return template;
        }
        panic!("This shouldn't happen");
    } else {
        panic!("Failed to load {}", filename);
    }
}

pub struct VoxTemplate {
    pub size: ModelSize,
    pub voxels: VoxMap,
}

impl VoxTemplate {
    pub fn merge(&mut self, other: &VoxTemplate) {
        if self.size != other.size {
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

pub fn build_palette_png() {
    use image::{ImageBuffer, Rgb};
    if Path::new("assets/vox/palette.png").exists() {
        return;
    }
    let mut image = ImageBuffer::<Rgb<u8>, std::vec::Vec<u8>>::new(64, 64);
    for (index, color_bytes) in dot_vox::DEFAULT_PALETTE.iter().enumerate() {
        let r: u8 = ((color_bytes & 0x00ff0000) >> 16) as u8;
        let g: u8 = ((color_bytes & 0x0000ff00) >> 8) as u8;
        let b: u8 = (color_bytes & 0x000000ff) as u8;

        println!("{}", index);
        let x = ((index % 16)*4) as u32;
        let y = ((index / 16)*4) as u32;

        for iy in 0..4 {
            for ix in 0..4 {
                *image.get_pixel_mut(x+ix, y+iy) = image::Rgb([r,g,b]);
            }
        }
    }
    image.save("assets/vox/palette.png").unwrap();
}