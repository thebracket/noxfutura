use super::camera::Camera;
use crate::engine::uniforms::UniformBlock;
use ultraviolet::Mat4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Uniforms {
    pub view_proj: Mat4,
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}
impl UniformBlock for Uniforms {}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            view_proj: ultraviolet::mat::Mat4::identity(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix();
    }
}
