mod float_buffer;
use float_buffer::FloatBuffer;

pub struct Buffers {
    buffers: Vec<FloatBuffer<f32>>
}

impl Buffers {
    pub fn new() -> Self {
        Self {
            buffers: Vec::new()
        }
    }

    pub fn init_buffer(&mut self, layout: &[usize], capacity: usize, usage: wgpu::BufferUsage) -> usize {
        let idx = self.buffers.len();
        let buf = FloatBuffer::new(layout, capacity, usage);
        self.buffers.push(buf);
        idx
    }

    pub fn get_buffer(&mut self, idx: usize) -> &mut FloatBuffer<f32> {
        &mut self.buffers[idx]
    }

    pub fn get_descriptor(&mut self, idx: usize) -> wgpu::VertexBufferDescriptor {
        println!("Get buffer descriptor: {}", idx);
        self.buffers[idx].descriptor()
    }
}