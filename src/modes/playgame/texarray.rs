use std::fs;
use std::collections::HashMap;

const TEXTURE_SIZE : usize = 256;
const ATLAS_COLS : usize = 16;
const ATLAS_W : u32 = (ATLAS_COLS * TEXTURE_SIZE) as u32;
const ATLAS_H : u32 = (ATLAS_COLS * TEXTURE_SIZE) as u32;

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

        // Make an atlas
        let atlas_rows = 16;
        println!("{} rows", atlas_rows);
        let mut atlas_data = image::DynamicImage::new_rgba8(ATLAS_W, ATLAS_H);

        texture_filenames.iter().enumerate().for_each(|(i, image_filename)| {
            println!("Loading {} for Atlas, image #{}", image_filename, i);
            let img = image::open(&image_filename).unwrap().into_rgba();
            let x = (i % ATLAS_COLS) * TEXTURE_SIZE;
            let y = (i / ATLAS_COLS) * TEXTURE_SIZE;
            image::imageops::overlay(&mut atlas_data, &img, x as u32, y as u32);
        });
        //image::save_buffer("atlas.png", &atlas_data.raw_pixels(), ATLAS_W, ATLAS_H, image::ColorType::RGBA(8));

        // Build the texture
        let size = wgpu::Extent3d {
            width: ATLAS_W as u32,
            height: ATLAS_H as u32,
            depth: 1,
        };
        let texture = context.device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        let buffer = context.device.create_buffer_with_data(&atlas_data.raw_pixels(), wgpu::BufferUsage::COPY_SRC);

        let mut encoder = context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("texture_buffer_copy_encoder"),
        });

        encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &buffer,
                offset: 0,
                bytes_per_row: 4 * ATLAS_W as u32,
                rows_per_image: ATLAS_H as u32,
            },
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                array_layer: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            size,
        );

        let cmd_buffer = encoder.finish();
        context.queue.submit(&[cmd_buffer]);

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
        Ok(
            Self {
                texture,
                view,
                sampler,
                dimensions : (ATLAS_W as usize, ATLAS_H as usize, 1),
                tex_map
            }
        )
    }
}
