#![allow(dead_code)]
use ultraviolet::{
    mat::Mat4,
    vec::{Vec3, Vec4},
};

pub struct Camera {
    eye: Vec3,
    target: Vec3,
    up: Vec3,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            eye: (0.0, 1.0, 1.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: Vec3::unit_y(),
            aspect: width as f32 / height as f32,
            fovy: 0.785398,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    pub fn build_view_projection_matrix(&self) -> Mat4 {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let opengl_to_wgpu_matrix : Mat4 = Mat4::new(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 0.5, 0.0),
            Vec4::new(0.0, 0.0, 0.5, 1.0),
        );

        let view = Mat4::look_at(self.eye, self.target, self.up);
        let proj =
            ultraviolet::projection::perspective_gl(self.fovy, self.aspect, self.znear, self.zfar);
        opengl_to_wgpu_matrix * proj * view
    }
}
