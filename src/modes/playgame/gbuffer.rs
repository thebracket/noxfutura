use crate::engine::Context;

pub struct GBuffer {
    pub albedo : GBufferTarget,
    pub normal : GBufferTarget,
    pub pbr : GBufferTarget,
    pub coords : GBufferTarget
}

impl GBuffer {
    pub fn new(context: &Context) -> Self {
        Self {
            albedo : GBufferTarget::make_texture(context, "Albedo"),
            normal : GBufferTarget::make_texture(context, "Normal"),
            pbr : GBufferTarget::make_texture(context, "PBR"),
            coords : GBufferTarget::make_texture(context, "Coords"),
        }
    }
}

pub struct GBufferTarget {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub dimensions: (usize, usize, usize),
}

impl GBufferTarget {
    pub fn make_texture(context: &Context, label: &str) -> Self {
        let size = wgpu::Extent3d {
            width: context.size.width as u32,
            height: context.size.height as u32,
            depth: 1,
        };
        let texture = context.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: context.swapchain_format,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        });

        let view = texture.create_default_view();
        let sampler = context.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: wgpu::CompareFunction::Always,
        });

        // Return something useful
        Self{
            texture,
            view,
            sampler,
            dimensions: (context.size.width as usize, context.size.height as usize, 1),
        }
    }
}