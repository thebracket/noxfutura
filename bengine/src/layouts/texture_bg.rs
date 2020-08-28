use crate::Textures;
use crate::RENDER_CONTEXT;

pub fn simple_texture_bg(textures: &Textures, layout: &wgpu::BindGroupLayout, texture_id: usize) -> wgpu::BindGroup {
    RENDER_CONTEXT.read().as_ref().unwrap()
    .device
        .create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(
                        textures.get_view(texture_id)
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(
                        textures.get_sampler(texture_id)
                    ),
                },
            ],
            label: Some("diffuse_bind_group"),
        }
    )
}