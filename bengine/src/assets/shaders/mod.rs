mod loader;
use parking_lot::RwLock;
use wgpu::ShaderModule;

lazy_static! {
    pub static ref SHADERS: RwLock<Shaders> = RwLock::new(Shaders::new());
}

pub struct Shaders {
    shaders: Vec<ShaderModule>,
}

impl Shaders {
    pub(crate) fn new() -> Self {
        Self {
            shaders: Vec::new(),
        }
    }

    pub fn register_include(&mut self, source: wgpu::ShaderModuleSource) -> usize {
        let sm = loader::from_spv(source);
        let idx = self.shaders.len();
        self.shaders.push(sm);
        idx
    }

    pub fn get_module(&self, id: usize) -> &wgpu::ShaderModule {
        &self.shaders[id]
    }
}
