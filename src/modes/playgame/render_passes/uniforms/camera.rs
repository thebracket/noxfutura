use nox_components::*;
use ultraviolet::Mat4;
use ultraviolet::Vec3;

pub struct Camera {
    pub eye: Vec3,
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
            eye: (256.0, 256.0, 128.0).into(),
            target: (128.0, 0.0, 128.0).into(),
            up: Vec3::unit_y(),
            aspect: width as f32 / height as f32,
            fovy: 0.785398,
            znear: 0.01,
            zfar: 512.0,
        }
    }

    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at(self.eye, self.target, self.up);
        let proj =
            ultraviolet::projection::perspective_gl(self.fovy, self.aspect, self.znear, self.zfar);
        proj * view
    }

    pub fn update(&mut self, pos: &Position, opts: &CameraOptions, width: u32, height: u32) {
        self.target= pos.as_vec3_glspace();
        self.aspect = width as f32 / height as f32;
        match opts.mode {
            CameraMode::TopDown => {
                self.eye = pos.as_vec3_glspace() + Vec3::new(0.0, opts.zoom_level as f32, opts.zoom_level as f32 / 3.0);
            }
            CameraMode::Front => {
                self.eye = pos.as_vec3_glspace() + Vec3::new(0.0, opts.zoom_level as f32, 0.1);
            }
            CameraMode::DiagonalNW => {
                self.eye = pos.as_vec3_glspace() + Vec3::new(opts.zoom_level as f32, opts.zoom_level as f32, opts.zoom_level as f32);
            }
            CameraMode::DiagonalNE => {
                self.eye = pos.as_vec3_glspace() + Vec3::new(-opts.zoom_level as f32, opts.zoom_level as f32, opts.zoom_level as f32);
            }
            CameraMode::DiagonalSW => {
                self.eye = pos.as_vec3_glspace() + Vec3::new(opts.zoom_level as f32, opts.zoom_level as f32, -opts.zoom_level as f32);
            }
            CameraMode::DiagonalSE => {
                self.eye = pos.as_vec3_glspace() + Vec3::new(-opts.zoom_level as f32, opts.zoom_level as f32, -opts.zoom_level as f32);
            }
        }

        // Sun testing
        //self.eye = (128.5, 512.0, 128.0).into();
        //self.target = (128.0, 128.0, 128.0).into();
    }
}
