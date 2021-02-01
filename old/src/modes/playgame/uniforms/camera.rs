use bengine::gpu::util::DeviceExt;
use bengine::uv::{Mat4, Vec3};
use bengine::*;
use nox_components::*;

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
            znear: 0.1,
            zfar: 256.0,
        }
    }

    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at(self.eye, self.target, self.up);
        let proj = bengine::uv::projection::perspective_wgpu_dx(
            self.fovy,
            self.aspect,
            self.znear,
            self.zfar,
        );
        proj * view
    }

    pub fn update(&mut self, pos: &Position, opts: &CameraOptions, width: u32, height: u32) {
        self.target = pos.as_vec3_glspace();
        self.aspect = width as f32 / height as f32;
        match opts.mode {
            CameraMode::TopDown => {
                self.eye = pos.as_vec3_glspace()
                    + Vec3::new(0.0, opts.zoom_level as f32, opts.zoom_level as f32 / 3.0);
            }
            CameraMode::Front => {
                self.eye = pos.as_vec3_glspace() + Vec3::new(0.0, opts.zoom_level as f32, 0.1);
            }
            CameraMode::DiagonalNW => {
                self.eye = pos.as_vec3_glspace()
                    + Vec3::new(
                        opts.zoom_level as f32,
                        opts.zoom_level as f32,
                        opts.zoom_level as f32,
                    );
            }
            CameraMode::DiagonalNE => {
                self.eye = pos.as_vec3_glspace()
                    + Vec3::new(
                        -opts.zoom_level as f32,
                        opts.zoom_level as f32,
                        opts.zoom_level as f32,
                    );
            }
            CameraMode::DiagonalSW => {
                self.eye = pos.as_vec3_glspace()
                    + Vec3::new(
                        opts.zoom_level as f32,
                        opts.zoom_level as f32,
                        -opts.zoom_level as f32,
                    );
            }
            CameraMode::DiagonalSE => {
                self.eye = pos.as_vec3_glspace()
                    + Vec3::new(
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

#[repr(C)]
pub struct CameraUniform {
    data: UniformData,
    pub uniform_buffer: gpu::Buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UniformData {
    view_proj: Mat4,
    rot_angle: f32,
}

unsafe impl bytemuck::Zeroable for UniformData {}
unsafe impl bytemuck::Pod for UniformData {}

impl CameraUniform {
    pub fn new() -> Self {
        let dcl = RENDER_CONTEXT.read();
        let dc = dcl.as_ref().unwrap();
        let data = UniformData {
            view_proj: Mat4::identity(),
            rot_angle: 0.0,
        };
        let uniform_buffer = dc
            .device
            .create_buffer_init(&gpu::util::BufferInitDescriptor {
                label: Some("PCUniforms"),
                contents: bytemuck::cast_slice(&[data]),
                usage: gpu::BufferUsage::UNIFORM | gpu::BufferUsage::COPY_DST,
            });
        Self {
            data,
            uniform_buffer,
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.data.view_proj = camera.build_view_projection_matrix();
        self.data.rot_angle += 0.001;

        let dcl = RENDER_CONTEXT.read();
        let dc = dcl.as_ref().unwrap();
        dc.queue
            .write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.data]));
    }
}
