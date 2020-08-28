mod textures;
mod shaders;
mod float_buffer;

pub use textures::TEXTURES;
pub use shaders::SHADERS;
pub use float_buffer::FloatBuffer;

pub fn make_empty_buffer(layout: &[usize], capacity: usize, usage: wgpu::BufferUsage) -> FloatBuffer<f32> {
    FloatBuffer::<f32>::new(layout, capacity, usage)
}

pub fn make_buffer_with_data(layout: &[usize], capacity: usize, usage: wgpu::BufferUsage, data: &[f32]) -> FloatBuffer<f32> {
    let mut buf = FloatBuffer::<f32>::new(layout, capacity, usage);
    buf.add_slice(data);
    buf.build();
    buf
}