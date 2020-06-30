use crate::engine::DEVICE_CONTEXT;

pub struct GBuffer {
    pub albedo: GBufferTarget,
    pub normal: GBufferTarget,
    pub pbr: GBufferTarget,
    pub coords: GBufferTarget,
}

impl GBuffer {
    pub fn new() -> Self {
        let swap_format = DEVICE_CONTEXT.read().as_ref().unwrap().swapchain_format;

        let albedo = GBufferTarget::make_texture("Albedo", swap_format);
        let normal = GBufferTarget::make_texture("Normal", wgpu::TextureFormat::Rgba32Float);
        let pbr = GBufferTarget::make_texture("PBR", swap_format);
        let coords = GBufferTarget::make_texture("Coords", wgpu::TextureFormat::Rgba32Float);

        Self {
            albedo,
            normal,
            pbr,
            coords,
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
    pub fn make_texture(label: &str, format: wgpu::TextureFormat) -> Self {
        let mut ctx_lock = DEVICE_CONTEXT.write();
        let context = ctx_lock.as_mut().unwrap();

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
            format,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        });

        let view = texture.create_default_view();
        let sampler = context.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: wgpu::CompareFunction::Always,
        });

        // Return something useful
        Self {
            texture,
            view,
            sampler,
            dimensions: (context.size.width as usize, context.size.height as usize, 1),
        }
    }
}
