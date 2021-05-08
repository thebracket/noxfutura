mod shader;
pub use shader::Shader;
mod texture;
mod vertex_buffer;
use lazy_static::*;
use parking_lot::RwLock;
use std::collections::HashMap;
pub use texture::Texture;
pub use vertex_buffer::VertexBuffer;

lazy_static! {
    pub static ref ASSETS: RwLock<Assets> = RwLock::new(Assets::new());
}

pub struct Assets {
    pub shaders: Vec<Shader>,
    pub shader_index: HashMap<String, usize>,
    pub vertex_buffers: Vec<VertexBuffer>,
    pub vertex_index: HashMap<String, usize>,
    pub textures: Vec<Texture>,
    pub texture_index: HashMap<String, usize>,
}

impl Assets {
    pub fn new() -> Self {
        Self {
            shaders: Vec::new(),
            shader_index: HashMap::new(),
            vertex_buffers: Vec::new(),
            vertex_index: HashMap::new(),
            textures: Vec::new(),
            texture_index: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_shader_from_source<S: ToString, S1: ToString>(
        &mut self,
        name: S1,
        vertex_source: S,
        frag_source: S,
    ) {
        let idx = self.shaders.len();
        let shader = Shader::from_source(name.to_string(), vertex_source, frag_source);
        self.shaders.push(shader);
        self.shader_index.insert(name.to_string(), idx);
    }

    pub fn add_shader_from_file<S: ToString, S1: ToString>(
        &mut self,
        name: S1,
        vertex_source: S,
        frag_source: S,
    ) {
        let idx = self.shaders.len();
        let shader = Shader::from_source_file(name.to_string(), vertex_source, frag_source);
        self.shaders.push(shader);
        self.shader_index.insert(name.to_string(), idx);
    }

    #[allow(dead_code)]
    pub fn shader_handle(&self, shader: &str) -> usize {
        self.shader_index[shader]
    }

    pub fn add_buffer_from_slice<S: ToString>(&mut self, name: S, layout: &[usize], data: &[f32]) {
        let n = name.to_string();
        let idx = self.vertex_buffers.len();
        let buffer = VertexBuffer::new(name, layout, data);
        self.vertex_buffers.push(buffer);
        self.vertex_index.insert(n, idx);
    }

    pub fn buffer_handle(&self, buffer: &str) -> usize {
        self.vertex_index[buffer]
    }

    pub fn add_texture_from_file<S: ToString, S1: ToString>(&mut self, name: S, filename: S1) {
        let n = name.to_string();
        let idx = self.textures.len();
        let tex = Texture::from_file(n.to_string(), filename);
        self.textures.push(tex);
        self.texture_index.insert(n, idx);
    }

    pub fn texture_handle(&self, texture: &str) -> usize {
        self.texture_index[texture]
    }
}

pub fn load_minimal_2d() {
    let mut asset_lock = ASSETS.write();
    asset_lock.add_shader_from_file(
        "simple2d",
        "resources/shader/simple2d/simple2d.vert",
        "resources/shader/simple2d/simple2d.frag",
    );
    asset_lock.add_buffer_from_slice(
        "background_quad",
        &[2, 2],
        &[
            -1.0, 1.0, 0.0, 0.0, -1.0, -1.0, 0.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 0.0, 0.0,
            1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0,
        ],
    );
    asset_lock.add_texture_from_file("background_logo", "resources/img/background_image.png");
}
