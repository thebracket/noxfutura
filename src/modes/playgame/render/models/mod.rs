use bengine::*;
use crate::modes::Palette;
use gpu::util::DeviceExt;

pub struct Models {
    pub vertex_buffer: FloatBuffer<f32>,
    pub index_buffer: gpu::Buffer,
    pub index_length: u32,
    pub model_map: Vec<ModelIndex>
}

pub struct ModelIndex {
    pub start: u16,
    pub end: u16
}

impl Models {
    pub fn load_models(palette: &Palette) -> Self {
        use crate::RAWS;

        let mut model_map = Vec::new();
        let rlock = RAWS.read();
        let mut index_buf = Vec::<u16>::new();
        let mut vertex_buffer = FloatBuffer::<f32>::new(&[3, 3, 1], 100, gpu::BufferUsage::VERTEX);
        let mut index_base = 0;

        for model_info in rlock.obj_models.models.iter() {

            let (models, _materials) = tobj::load_obj(
                &model_info.file,
                false
            )
            .unwrap();

            let start = index_base;

            for m in models.iter() {
                let mat_finder = rlock.obj_models.colors.iter().find(|c| c.tag == m.name).unwrap();
                let mat_index = palette.find_palette(
                    mat_finder.r,
                    mat_finder.g,
                    mat_finder.b,
                );

                let indices: Vec<u16> = m
                    .mesh
                    .indices
                    .iter()
                    .map(|i| *i as u16 + index_base)
                    .collect();
                let mut vertices: Vec<f32> = Vec::new();
                for i in 0..m.mesh.positions.len() / 3 {
                    vertices.push(m.mesh.positions[i * 3] * model_info.scale);
                    vertices.push(m.mesh.positions[(i * 3) + 1] * model_info.scale);
                    vertices.push(m.mesh.positions[(i * 3) + 2] * model_info.scale);
                    vertices.push(m.mesh.normals[i * 3] * model_info.scale);
                    vertices.push(m.mesh.normals[(i * 3) + 1] * model_info.scale);
                    vertices.push(m.mesh.normals[(i * 3) + 2] * model_info.scale);
                    vertices.push(mat_index as f32);
                }
                index_base += m.mesh.positions.len() as u16 / 3;
                vertex_buffer.add_slice(&vertices);
                index_buf.extend_from_slice(&indices);
                model_map.push(ModelIndex{
                    start, end: index_buf.len() as u16
                });
            }
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
            model_map
        }
    }
}