use cgmath::{EuclideanSpace, Matrix4, Point3, SquareMatrix, Vector3};
mod camera;
pub use camera::*;
use bengine::*;
use bengine::gpu::util::DeviceExt;

#[repr(C)]
pub struct Uniforms {
    data: UniformData,
    pub uniform_buffer: gpu::Buffer
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UniformData {
    view_proj: Matrix4<f32>,
    rot_angle: f32,
}

unsafe impl bytemuck::Zeroable for UniformData {}
unsafe impl bytemuck::Pod for UniformData {}

impl Uniforms {
    pub fn new() -> Self {
        let dcl = RENDER_CONTEXT.read();
        let dc = dcl.as_ref().unwrap();
        let data = UniformData {
            view_proj: Matrix4::identity(),
            rot_angle: 0.0
        };
        let uniform_buffer = dc.device.create_buffer_init(
            &gpu::util::BufferInitDescriptor{
                label: Some("WGUniforms"),
                contents: bytemuck::cast_slice(&[data]),
                usage: gpu::BufferUsage::UNIFORM | gpu::BufferUsage::COPY_DST,
            }
        );
        Self {
            data,
            uniform_buffer
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.data.view_proj = camera.build_view_projection_matrix();
        self.data.rot_angle += 0.001;

        let dcl = RENDER_CONTEXT.read();
        let dc = dcl.as_ref().unwrap();
        dc.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.data]));
    }
}