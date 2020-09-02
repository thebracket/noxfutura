use cgmath::{Vector3, Matrix4, Point3};
use cgmath::EuclideanSpace;

#[cfg_attr(rustfmt, rustfmt_skip)]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct Camera {
    eye: Vector3<f32>,
    target: Vector3<f32>,
    up: Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            eye: (0.0, 2.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: Vector3::unit_y(),
            aspect: width as f32 / height as f32,
            fovy: 0.785398,
            znear: 0.01,
            zfar: 500.0,
        }
    }

    pub fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at(
            Point3::from_vec(self.eye),
            Point3::from_vec(self.target),
            self.up,
        );
        let proj = cgmath::perspective(cgmath::Rad(self.fovy), self.aspect, self.znear, self.zfar);
        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}