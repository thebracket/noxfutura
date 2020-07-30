use nox_components::*;
use cgmath::{Matrix4, Vector3, Point3, EuclideanSpace};

pub struct Camera {
    pub eye: Vector3<f32>,
    target: Vector3<f32>,
    up: Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            eye: (256.0, 256.0, 128.0).into(),
            target: (128.0, 0.0, 128.0).into(),
            up: Vector3::unit_y(),
            aspect: width as f32 / height as f32,
            fovy: 0.785398,
            znear: 0.1,
            zfar: 256.0,
        }
    }

    pub fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at(
            Point3::from_vec(self.eye), 
            Point3::from_vec(self.target), 
            self.up
        );
        let proj = cgmath::perspective(cgmath::Rad(self.fovy), self.aspect, self.znear, self.zfar);
        OPENGL_TO_WGPU_MATRIX * proj * view
    }

    pub fn update(&mut self, pos: &Position, opts: &CameraOptions, width: u32, height: u32) {
        self.target = pos.as_vec3_glspace();
        self.aspect = width as f32 / height as f32;
        match opts.mode {
            CameraMode::TopDown => {
                self.eye = pos.as_vec3_glspace()
                    + Vector3::new(0.0, opts.zoom_level as f32, opts.zoom_level as f32 / 3.0);
            }
            CameraMode::Front => {
                self.eye = pos.as_vec3_glspace() + Vector3::new(0.0, opts.zoom_level as f32, 0.1);
            }
            CameraMode::DiagonalNW => {
                self.eye = pos.as_vec3_glspace()
                    + Vector3::new(
                        opts.zoom_level as f32,
                        opts.zoom_level as f32,
                        opts.zoom_level as f32,
                    );
            }
            CameraMode::DiagonalNE => {
                self.eye = pos.as_vec3_glspace()
                    + Vector3::new(
                        -opts.zoom_level as f32,
                        opts.zoom_level as f32,
                        opts.zoom_level as f32,
                    );
            }
            CameraMode::DiagonalSW => {
                self.eye = pos.as_vec3_glspace()
                    + Vector3::new(
                        opts.zoom_level as f32,
                        opts.zoom_level as f32,
                        -opts.zoom_level as f32,
                    );
            }
            CameraMode::DiagonalSE => {
                self.eye = pos.as_vec3_glspace()
                    + Vector3::new(
                        -opts.zoom_level as f32,
                        opts.zoom_level as f32,
                        -opts.zoom_level as f32,
                    );
            }
        }

        // Sun testing
        //self.eye = (128.5, 512.0, 128.0).into();
        //self.target = (128.0, 128.0, 128.0).into();
    }
}
