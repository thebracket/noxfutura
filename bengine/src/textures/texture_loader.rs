use super::TextureRef;
use wgpu::{Device, Queue};
use image::GenericImageView;

pub(crate) fn from_bytes(
    device: &Device,
    queue: &Queue,
    bytes: &[u8],
    label: &str,
) -> Result<TextureRef, failure::Error> {
    let img = image::load_from_memory(bytes)?;
    from_image(device, queue, &img, Some(label))
}

pub(crate) fn from_image(
    device: &Device,
    queue: &Queue,
    img: &image::DynamicImage,
    label: Option<&str>,
) -> Result<TextureRef, failure::Error> {
    let rgba = img.as_rgba8().unwrap();
    let dimensions = img.dimensions();

    let size = wgpu::Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        depth: 1,
    };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label,
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
    });

    queue.write_texture(
        wgpu::TextureCopyView{
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO
        },
        &rgba,
        wgpu::TextureDataLayout{
            offset: 0,
            bytes_per_row: 4 * size.width,
            rows_per_image: size.height
        },
        size
    );

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
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
        anisotropy_clamp: None
    });

    Ok(
        TextureRef {
            texture,
            view,
            sampler,
        }
    )
}