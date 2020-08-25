use wgpu::{Device, Queue};
use crate::{Textures, Shaders, ShaderType, Buffers};

pub struct Initializer<'a> {
    device: &'a Device,
    queue: &'a Queue,
    textures: &'a mut Textures,
    shaders: &'a mut Shaders,
    buffers: &'a mut Buffers
}

impl<'a> Initializer<'a> {
    pub(crate) fn new(
        device: &'a Device,
        queue: &'a Queue,
        textures: &'a mut Textures,
        shaders: &'a mut Shaders,
        buffers: &'a mut Buffers
    ) -> Self {
        Self {
            device,
            queue,
            textures,
            shaders,
            buffers
        }
    }

    pub fn load_texture_from_bytes(&mut self, bytes: &[u8]) -> usize {
        self.textures.load_texture_from_bytes(
            self.device,
            self.queue,
            bytes,
            "Background"
        )
    }

    pub fn load_shader_from_file<S: ToString>(&mut self, filename: S, shader_type: ShaderType) -> usize {
        self.shaders.register(filename, shader_type, self.device)
    }

    pub fn make_empty_buffer(&mut self, layout: &[usize], capacity: usize, usage: wgpu::BufferUsage) -> usize {
        self.buffers.init_buffer(layout, capacity, usage)
    }

    pub fn make_buffer_with_data(&mut self, layout: &[usize], capacity: usize, usage: wgpu::BufferUsage, data: &[f32]) -> usize {
        let idx = self.buffers.init_buffer(layout, capacity, usage);
        let mut buf = self.buffers.get_buffer(idx);
        buf.add_slice(data);
        buf.build(self.device);
        idx
    }
}