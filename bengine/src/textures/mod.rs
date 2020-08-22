mod depth_texture;
mod texture_loader;
pub(crate) use depth_texture::create_depth_texture;
use wgpu::{Device, Queue};
use winit::dpi::PhysicalSize;

pub struct TextureRef {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

pub struct Textures {
    textures: Vec<TextureRef>
}

impl Textures {
    pub fn new() -> Self {
        Textures{
            textures: Vec::new()
        }
    }

    pub fn register_new_depth_texture(&mut self, device: &Device, size: PhysicalSize<u32>, label: &str) -> usize {
        let tex = create_depth_texture(device, size, label);
        let id = self.textures.len();
        self.textures.push(tex);
        id
    }

    pub fn replace_depth_texture(&mut self, id: usize, device: &Device, size: PhysicalSize<u32>, label: &str) -> usize {
        let tex = create_depth_texture(device, size, label);
        self.textures[id] = tex;
        id
    }

    pub fn load_texture_from_bytes(&mut self, device: &Device, queue: &Queue, bytes: &[u8], label: &str) -> usize {
        let tex = texture_loader::from_bytes(device, queue, bytes, label).unwrap();
        let id = self.textures.len();
        self.textures.push(tex);
        id
    }

    pub fn load_texture_from_image(&mut self, device: &Device, queue: &Queue, image: &image::DynamicImage, label: &str) -> usize {
        let tex = texture_loader::from_image(device, queue, image, Some(label)).unwrap();
        let id = self.textures.len();
        self.textures.push(tex);
        id
    }
}