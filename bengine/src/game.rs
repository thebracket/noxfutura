use crate::core::Core;

pub trait BEngineGame {
    fn init(&mut self);
    fn tick(&mut self, core: &mut Core) -> bool;
    fn get_mouse_buffer(&self) -> Option<&wgpu::Buffer>;
    fn on_resize(&mut self);
}
