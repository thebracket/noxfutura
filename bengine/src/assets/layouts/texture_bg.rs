use crate::RENDER_CONTEXT;
use crate::TEXTURES;

pub fn simple_texture_bg(layout: &wgpu::BindGroupLayout, texture_id: usize) -> wgpu::BindGroup {
    let textures = TEXTURES.read();
    RENDER_CONTEXT
        .read()
        .as_ref()
        .unwrap()
        .device
        .create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(textures.get_view(texture_id)),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(textures.get_sampler(texture_id)),
                },
            ],
            label: Some("diffuse_bind_group"),
        })
}
