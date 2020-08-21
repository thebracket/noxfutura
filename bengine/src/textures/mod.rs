mod depth_texture;
pub(crate) use depth_texture::create_depth_texture;

pub(crate) struct TextureRef {
    pub(crate) texture: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
    pub(crate) sampler: wgpu::Sampler,
}
