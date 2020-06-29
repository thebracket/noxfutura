use legion::prelude::*;
use ultraviolet::Vec3;
use crate::engine::uniforms::UniformBlock;
use crate::components::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LightUniforms {
    pub camera_position: [f32; 4],
    pub lights : [LightInfo; 32],
}

unsafe impl bytemuck::Pod for LightUniforms {}
unsafe impl bytemuck::Zeroable for LightUniforms {}
impl UniformBlock for LightUniforms {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct LightInfo {
    pub pos: [f32; 4],
    pub color: [f32; 4],
}

impl LightInfo {
    fn new() -> Self {
        Self {
            pos: [0.0, 0.0, 0.0, 0.0],
            color: [0.0, 0.0, 0.0, 0.0],
        }
    }
}

unsafe impl bytemuck::Pod for LightInfo {}
unsafe impl bytemuck::Zeroable for LightInfo {}

impl LightUniforms {
    pub fn new() -> Self {
        Self {
            camera_position: [0.0, 0.0, 0.0, 0.0],
            lights: [LightInfo::new(); 32]
        }
    }

    pub fn update(&mut self, ecs: &World, sun_pos: Vec3, camera_pos: Vec3) {
        self.camera_position = vec_to_float(&camera_pos);
        self.lights[0].pos = [ sun_pos.x, sun_pos.y, sun_pos.z, 512.0 ];
        self.lights[0].color = [ 1.0, 1.0, 1.0, 1.0 ];

        self.lights.iter_mut().skip(1).for_each(|l| {
            l.pos = [ 0.0, 0.0, 0.0, 0.0];
            l.color = [0.0, 0.0, 0.0, 0.0];
        });

        let mut index = 1;
        let light_query = <(Read<Position>, Read<Light>)>::query();
        light_query.iter(ecs).for_each(|(pos, light)| {
            if index < 32 {
                self.lights[index].color = [ light.color.0, light.color.1, light.color.2, 0.0 ];
                self.lights[index].pos = [ pos.x as f32 + 0.5, pos.z as f32 + 0.5, pos.y as f32 + 0.5, light.radius as f32 ]
            }
            index += 1;
        });
        //println!("{:#?}", self.lights);
    }
}

#[inline(always)]
fn vec_to_float(v : &Vec3) -> [f32; 4] {
    [v.x, v.y, v.z, 0.0]
}