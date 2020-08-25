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
        self.buffers.push(FloatBuffer::new(layout, capacity, usage));
        idx
    }

    pub fn get_buffer(&mut self, idx: usize) -> &mut FloatBuffer<f32> {
        &mut self.buffers[idx]
    }
}