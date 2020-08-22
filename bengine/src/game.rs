use crate::core::Core;
use crate::{Textures, Shaders};
use wgpu::{Device, Queue};

pub trait BEngineGame {
    fn init(&mut self, device: &Device, queue: &Queue, textures: &mut Textures, shaders: &mut Shaders);
    fn tick(&mut self, core: &mut Core) -> bool;
}
