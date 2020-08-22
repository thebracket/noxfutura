mod loader;
use wgpu::{Device, ShaderModule};
pub use loader::ShaderType;

pub struct Shaders {
    shaders: Vec<ShaderModule>
}

impl Shaders {
    pub(crate) fn new() -> Self {
        Self {
            shaders: Vec::new()
        }
    }

    pub fn register<S: ToString>(&mut self, filename: S, shader_type: ShaderType, device: &Device) -> usize {
        let sm = loader::from_source(filename, shader_type, device);
        let idx = self.shaders.len();
        self.shaders.push(sm);
        idx
    }
}