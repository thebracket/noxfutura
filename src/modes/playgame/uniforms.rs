use ultraviolet::Vec3;
use ultraviolet::Mat4;
use crate::engine::uniforms::UniformBlock;
use super::Camera;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Uniforms {
    pub view_proj: Mat4,
    pub sun_pos: Vec3,
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}
impl UniformBlock for Uniforms {}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            view_proj: ultraviolet::mat::Mat4::identity(),
            sun_pos: (128.0, 256.0, 128.0).into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera, counter: usize) {
        use bracket_geometry::prelude::*;
        let angle = (counter % 360) as f32;
        let p = project_angle(Point::new(0,0), 256.0, Degrees::new(angle));
        self.sun_pos = (
            p.x as f32,
            p.y as f32,
            128.0
        ).into();
        //println!("{:?}, {}", p, counter);

        self.view_proj = camera.build_view_projection_matrix();
    }
}
