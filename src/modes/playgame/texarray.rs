use std::fs;
use std::collections::HashMap;

pub struct TextureArray {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub dimensions : (usize, usize, usize),
    pub tex_map : HashMap::<String, usize>
}

impl TextureArray {
    pub fn blank(
        context: &crate::engine::Context,
    ) -> Result<Self, failure::Error> {
        let label = Some("terraintex");
        let width = 256;
        let height = 256;

        let paths = fs::read_dir("resources/terrain/").unwrap();
        let mut texture_filenames = paths
            .map(|p| p.unwrap().path().display().to_string())
            .filter(|p| p.ends_with(".jpg"))
            .collect::<Vec<String>>();
        texture_filenames.sort();

        let mut tex_map = HashMap::<String, usize>::new();
        for (i,t) in texture_filenames.iter().enumerate() {
            let stubname = t.replace("resources/terrain/", "")
                .replace("-ao.jpg", "")
                .replace("-n.jpg", "")
                .replace("-r.jpg", "")
                .replace("-m.jpg", "")
                .replace("-t.jpg", "");
            if !tex_map.contains_key(&stubname) {
                tex_map.insert(stubname, i);
            }
        }
        println!("{:#?}", tex_map);

        let n_images = texture_filenames.len();

        let size = wgpu::Extent3d {
            width: width as u32,
            height: height as u32,
            depth: 1,
        };
        let texture = context.device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            array_layer_count: n_images as u32,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        for (i,filename) in texture_filenames.iter().enumerate() {
            println!("Loading {}", filename);
            let img = image::open(&filename).unwrap().resize_exact(256, 256, image::imageops::FilterType::Lanczos3);
            let rgba = img.into_rgba();

            let buffer = context.device.create_buffer_with_data(&rgba, wgpu::BufferUsage::COPY_SRC);

            let mut encoder = context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: None,
            });

            encoder.copy_buffer_to_texture(
                wgpu::BufferCopyView {
                    buffer: &buffer,
                    offset: 0,
                    bytes_per_row: 4 * width as u32,
                    rows_per_image: 1,
                },
                wgpu::TextureCopyView {
                    texture: &texture,
                    mip_level: 0,
                    array_layer: i as u32,
                    origin: wgpu::Origin3d::ZERO,
                },
                size,
            );

            let cmd_buffer = encoder.finish();
            context.queue.submit(&[cmd_buffer]);
        }

        let view = texture.create_default_view();
        let sampler = context.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: wgpu::CompareFunction::Always,
        });

        Ok(
            Self {
                texture,
                view,
                sampler,
                dimensions : (width, height, 1),
                tex_map
            }
        )
    }
}
