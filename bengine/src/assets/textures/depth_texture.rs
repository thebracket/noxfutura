use super::TextureRef;

const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

pub(crate) fn create_depth_texture(
    device: &wgpu::Device,
    size: winit::dpi::PhysicalSize<u32>,
    label: &str,
) -> TextureRef {
    let size = wgpu::Extent3d {
        width: size.width,
        height: size.height,
        depth: 1,
    };
    let desc = wgpu::TextureDescriptor {
        label: Some(label),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: DEPTH_FORMAT,
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT
            | wgpu::TextureUsage::SAMPLED
            | wgpu::TextureUsage::COPY_SRC,
    };
    let texture = device.create_texture(&desc);

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Nearest,
        lod_min_clamp: -100.0,
        lod_max_clamp: 100.0,
        compare: Some(wgpu::CompareFunction::LessEqual),
        ..Default::default()
    });

    TextureRef {
        texture,
        view,
        sampler,
    }
}
