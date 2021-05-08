use super::super::RENDER_CONTEXT;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsage, VertexAttribute, VertexBufferLayout,
};

pub struct VertexBuffer {
    pub name: String,
    pub attributes: Vec<VertexAttribute>,
    pub total_size: u64,
    pub buffer: Buffer,
}

impl VertexBuffer {
    pub fn new<S: ToString>(name: S, layout: &[usize], data: &[f32]) -> Self {
        let mut attributes = Vec::new();
        let mut cumulative_size = 0;
        for (i, size) in layout.iter().enumerate() {
            let attribute = VertexAttribute {
                offset: cumulative_size,
                shader_location: i as u32,
                format: match size {
                    1 => wgpu::VertexFormat::Float32,
                    2 => wgpu::VertexFormat::Float32x2,
                    3 => wgpu::VertexFormat::Float32x3,
                    4 => wgpu::VertexFormat::Float32x4,
                    _ => {
                        panic!("Vertices must be 1-4 floats");
                    }
                },
            };
            attributes.push(attribute);
            cumulative_size += (std::mem::size_of::<f32>() * size) as wgpu::BufferAddress;
        }

        let mut gpu_lock = RENDER_CONTEXT.write();
        let gpu = gpu_lock.as_mut().unwrap();
        let vertex_buffer = gpu.device.create_buffer_init(&BufferInitDescriptor {
            label: Some(&name.to_string()),
            contents: bytemuck::cast_slice(data),
            usage: BufferUsage::VERTEX,
        });

        VertexBuffer {
            name: name.to_string(),
            attributes,
            total_size: cumulative_size,
            buffer: vertex_buffer,
        }
    }

    pub fn descriptor(&self) -> VertexBufferLayout {
        VertexBufferLayout {
            array_stride: self.total_size,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &self.attributes,
        }
    }
}
