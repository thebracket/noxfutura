use crate::modes::Palette;
use bengine::*;
use gpu::util::DeviceExt;

pub struct Model {
    pub vertex_buffer: FloatBuffer<f32>,
    pub index_buffer: gpu::Buffer,
    pub index_length: u32,
}

impl Model {
    pub fn load(filename: &str, palette: &Palette) -> Self {
        let (models, _materials) = tobj::load_obj(filename, false).unwrap();

        // Format: position, normal, colour index
        let mut index_buf = Vec::<u16>::new();
        let mut vertex_buffer = FloatBuffer::<f32>::new(&[3, 3, 1], 100, gpu::BufferUsage::VERTEX);
        let mut index_base = 0;
        for m in models.iter() {
            let indices: Vec<u16> = m
                .mesh
                .indices
                .iter()
                .map(|i| *i as u16 + index_base)
                .collect();
            let mut vertices: Vec<f32> = Vec::new();
            let mat_index = match m.name.as_str() {
                "leaf" => palette.find_palette(0.0, 1.0, 0.0),
                "leaf_2" => palette.find_palette(0.0, 0.5, 0.0),
                "leaf_3" => palette.find_palette(0.0, 0.75, 0.0),
                "leaf_4" => palette.find_palette(0.0, 0.25, 0.0),
                "bark" => palette.find_palette(0.75, 0.6, 0.4),
                _ => {
                    println!("{}", m.name);
                    10
                }
            };
            for i in 0..m.mesh.positions.len() / 3 {
                vertices.push(m.mesh.positions[i * 3]);
                vertices.push(m.mesh.positions[(i * 3) + 1]);
                vertices.push(m.mesh.positions[(i * 3) + 2]);
                vertices.push(m.mesh.normals[i * 3]);
                vertices.push(m.mesh.normals[(i * 3) + 1]);
                vertices.push(m.mesh.normals[(i * 3) + 2]);
                vertices.push(mat_index as f32);
            }
            index_base += m.mesh.positions.len() as u16 / 3;
            vertex_buffer.add_slice(&vertices);
            index_buf.extend_from_slice(&indices);
        }

        let ctl = RENDER_CONTEXT.read();
        let ctx = ctl.as_ref().unwrap();
        let index_buffer = ctx
            .device
            .create_buffer_init(&gpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&index_buf),
                usage: gpu::BufferUsage::INDEX,
            });
        vertex_buffer.build();

        Self {
            vertex_buffer,
            index_buffer,
            index_length: index_buf.len() as u32,
        }
    }
}
