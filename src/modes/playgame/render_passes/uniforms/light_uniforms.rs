
use ultraviolet::Vec3;
use crate::engine::uniforms::UniformBlock;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LightUniforms {
    pub sun_pos: Vec3,
    pub sun_color: Vec3,
    pub camera_position: Vec3
}

unsafe impl bytemuck::Pod for LightUniforms {}
unsafe impl bytemuck::Zeroable for LightUniforms {}
impl UniformBlock for LightUniforms {}

impl LightUniforms {
    pub fn new() -> Self {
        Self {
            sun_pos: (128.0, 512.0, 128.0).into(),
            sun_color: (1.0, 1.0, 1.0).into(),
            camera_position: (0.0, 0.0, 0.0).into()
        }
    }

    pub fn update(&mut self, sun_pos: Vec3, camera_pos: Vec3) {
        self.sun_pos = sun_pos;
        self.camera_position = camera_pos;
        //println!("{:#?}", self.sun_pos);
        //println!("{:#?}", self.view_proj);
    }
}