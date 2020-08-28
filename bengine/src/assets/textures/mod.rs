mod depth_texture;
mod texture_loader;
pub(crate) use depth_texture::create_depth_texture;
use crate::RENDER_CONTEXT;
use parking_lot::RwLock;

lazy_static! {
    pub static ref TEXTURES : RwLock<Textures> = RwLock::new(Textures::new());
}

pub struct TextureRef {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

pub struct Textures {
    textures: Vec<TextureRef>
}

impl Textures {
    fn new() -> Self {
        Textures{
            textures: Vec::new()
        }
    }

    pub fn register_new_depth_texture(&mut self, label: &str) -> usize {
        let rcl = RENDER_CONTEXT.read();
        let rc = rcl.as_ref().unwrap();
        let tex = create_depth_texture(&rc.device, rc.size, label);
        let id = self.textures.len();
        self.textures.push(tex);
        id
    }

    pub fn replace_depth_texture(&mut self, id: usize, label: &str) -> usize {
        let rcl = RENDER_CONTEXT.read();
        let rc = rcl.as_ref().unwrap();
        let tex = create_depth_texture(&rc.device, rc.size, label);
        self.textures[id] = tex;
        id
    }

    pub fn load_texture_from_bytes(&mut self, bytes: &[u8], label: &str) -> usize {
        let tex = texture_loader::from_bytes(bytes, label).unwrap();
        let id = self.textures.len();
        self.textures.push(tex);
        id
    }

    pub fn load_texture_from_image(&mut self, image: &image::DynamicImage, label: &str) -> usize {
        let tex = texture_loader::from_image(image, Some(label)).unwrap();
        let id = self.textures.len();
        self.textures.push(tex);
        id
    }

    pub fn get_view(&self, id: usize) -> &wgpu::TextureView {
        &self.textures[id].view
    }

    pub fn get_sampler(&self, id: usize) -> &wgpu::Sampler {
        &self.textures[id].sampler
    }
}