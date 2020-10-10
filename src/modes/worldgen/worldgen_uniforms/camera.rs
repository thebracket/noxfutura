use bengine::uv::{Mat4, Vec3};

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
            eye: (0.0, 2.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: Vec3::unit_y(),
            aspect: width as f32 / height as f32,
            fovy: 0.785398,
            znear: 0.01,
            zfar: 500.0,
        }
    }

    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at(
            self.eye,
            self.target,
            self.up,
        );
        let proj = bengine::uv::projection::perspective_wgpu_dx(self.fovy, self.aspect, self.znear, self.zfar);
        proj * view
    }
}
