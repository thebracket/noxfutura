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
}