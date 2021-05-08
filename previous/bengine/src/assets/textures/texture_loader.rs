use super::TextureRef;
use crate::RENDER_CONTEXT;
use image::GenericImageView;

pub(crate) fn from_bytes(bytes: &[u8], label: &str) -> Result<TextureRef, failure::Error> {
    let img = image::load_from_memory(bytes)?;
    from_image(&img, Some(label))
}

pub(crate) fn from_image(
    img: &image::DynamicImage,
    label: Option<&str>,
) -> Result<TextureRef, failure::Error> {
    let rgba = img.as_rgba8().expect("Unable to get RGBA8 buffer");
    let dimensions = img.dimensions();

    let rcl = RENDER_CONTEXT.read();
    let rc = rcl.as_ref().unwrap();

    let size = wgpu::Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        depth: 1,
    };
    let texture = rc.device.create_texture(&wgpu::TextureDescriptor {
        label,
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
    });

    rc.queue.write_texture(
        wgpu::TextureCopyView {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        &rgba,
        wgpu::TextureDataLayout {
            offset: 0,
            bytes_per_row: 4 * size.width,
            rows_per_image: size.height,
        },
        size,
    );

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = rc.device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::Repeat,
        address_mode_v: wgpu::AddressMode::Repeat,
        address_mode_w: wgpu::AddressMode::Repeat,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        lod_min_clamp: -100.0,
        lod_max_clamp: 100.0,
        compare: None,
        label: None,
        anisotropy_clamp: None,
    });

    Ok(TextureRef {
        texture,
        view,
        sampler,
    })
}
