use std::collections::HashMap;
use std::fs;
use crate::engine::DEVICE_CONTEXT;
use crate::modes::loader_progress;

const TEXTURE_SIZE: usize = 256;
const ATLAS_COLS: usize = 16;
const ATLAS_W: u32 = (ATLAS_COLS * TEXTURE_SIZE) as u32;
const ATLAS_H: u32 = (ATLAS_COLS * TEXTURE_SIZE) as u32;

pub struct TextureArray {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub dimensions: (usize, usize, usize),
    pub tex_map: HashMap<String, usize>,
}

impl TextureArray {
    pub fn blank() -> Result<Self, failure::Error> {
        loader_progress(0.1, "Building texture maps", false);
        let label = Some("terraintex");

        let paths = fs::read_dir("resources/terrain/").unwrap();
        let mut texture_filenames = paths
            .map(|p| p.unwrap().path().display().to_string())
            .filter(|p| p.ends_with(".jpg"))
            .collect::<Vec<String>>();
        texture_filenames.sort();

        let mut tex_map = HashMap::<String, usize>::new();
        for (i, t) in texture_filenames.iter().enumerate() {
            let stubname = t
                .replace("resources/terrain/", "")
                .replace("-ao.jpg", "")
                .replace("-n.jpg", "")
                .replace("-r.jpg", "")
                .replace("-m.jpg", "")
                .replace("-t.jpg", "");
            assert!(!stubname.ends_with(".jpg"));
            if !tex_map.contains_key(&stubname) {
                tex_map.insert(stubname, i);
            }
        }

        // Make an atlas
        //let atlas_rows = 16;
        //println!("{} rows", atlas_rows);
        loader_progress(0.15, "Building an atlas", false);
        let mut atlas_data = [
            image::DynamicImage::new_rgba8(ATLAS_W, ATLAS_H),
            image::DynamicImage::new_rgba8(ATLAS_W / 2, ATLAS_H / 2),
            image::DynamicImage::new_rgba8(ATLAS_W / 4, ATLAS_H / 4),
            image::DynamicImage::new_rgba8(ATLAS_W / 8, ATLAS_H / 8),
            image::DynamicImage::new_rgba8(ATLAS_W / 16, ATLAS_H / 16),
            image::DynamicImage::new_rgba8(ATLAS_W / 32, ATLAS_H / 32),
            image::DynamicImage::new_rgba8(ATLAS_W / 64, ATLAS_H / 64),
            image::DynamicImage::new_rgba8(ATLAS_W / 128, ATLAS_H / 128),
        ];

        loader_progress(0.17, "Squashing things", false);
        let mut matmap = HashMap::<String, usize>::new();
        for (i, (k, _v)) in tex_map.iter().enumerate() {
            // i is index, k is stub-name, v is the index in the texture array and can be largely ignored?
            let albdeo_fn = format!("resources/terrain/{}-t.jpg", k);
            let normal_fn = format!("resources/terrain/{}-n.jpg", k);
            let rough_fn = format!("resources/terrain/{}-r.jpg", k);
            let ao_fn = format!("resources/terrain/{}-ao.jpg", k);
            let metal_fn = format!("resources/terrain/{}-m.jpg", k);

            matmap.insert(k.clone(), i * 3);

            //println!("Constructing {}: {}", i, k);
            let albedo = image::open(&albdeo_fn).unwrap();
            let normal = image::open(&normal_fn).unwrap();
            let rough = image::open(&rough_fn).unwrap();
            let ao = image::open(&ao_fn).unwrap();
            let metal = if std::path::Path::new(&metal_fn).exists() {
                image::open(&metal_fn).unwrap()
            } else {
                image::open("resources/metal-template.jpg").unwrap()
            };

            let albedo_mip = load_image_levels(&albedo);
            let normal_mip = load_image_levels(&normal);
            let rough_mip = load_image_levels(&rough);
            let ao_mip = load_image_levels(&ao);
            let metal_mip = load_image_levels(&metal);

            let mut tex_size = TEXTURE_SIZE as usize;
            for mip in 0..8 {
                // Paste the albedo
                let base_i = i * 3;
                let x = (base_i % ATLAS_COLS) * tex_size;
                let y = (base_i / ATLAS_COLS) * tex_size;
                image::imageops::overlay(
                    &mut atlas_data[mip],
                    &albedo_mip[mip],
                    x as u32,
                    y as u32,
                );

                // Paste the normal
                let x = ((base_i + 1) % ATLAS_COLS) * tex_size;
                let y = ((base_i + 1) / ATLAS_COLS) * tex_size;
                image::imageops::overlay(
                    &mut atlas_data[mip],
                    &normal_mip[mip],
                    x as u32,
                    y as u32,
                );

                let mut fancy = ao_mip[mip].clone();
                for iy in 0..tex_size as u32 {
                    for ix in 0..tex_size as u32 {
                        let aop = ao_mip[mip].get_pixel(ix, iy);
                        let roughp = rough_mip[mip].get_pixel(ix, iy);
                        let metalp = metal_mip[mip].get_pixel(ix, iy);
                        let p = image::Rgba::<u8>([aop[0], roughp[0], metalp[0], 255]);
                        fancy.put_pixel(ix, iy, p);
                    }
                }
                let x = ((base_i + 2) % ATLAS_COLS) * tex_size;
                let y = ((base_i + 2) / ATLAS_COLS) * tex_size;
                image::imageops::overlay(&mut atlas_data[mip], &fancy, x as u32, y as u32);
                tex_size /= 2;
            }
        }

        loader_progress(0.18, "Mapping Materials", false);
        {
            let mut rawlock = crate::raws::RAWS.write();
            let mats = rawlock.materials.materials.clone();
            rawlock.matmap.build(&matmap, &mats);
        }

        // Build the texture
        loader_progress(0.19, "Throwing data around", false);
        let size = wgpu::Extent3d {
            width: ATLAS_W as u32,
            height: ATLAS_H as u32,
            depth: 1,
        };
        let mut ctx = DEVICE_CONTEXT.write();
        let context = ctx.as_mut().unwrap();
        let texture = context.device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            array_layer_count: 1,
            mip_level_count: 8,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        const CAPACITY : usize = (4096*4096)+(2048+2048)+(1024*1024)+(512*512)+(256*256)+(128*128)+(64*64)+(32*32)+(16*16) * 4;
        let mut pixelbuf = Vec::<u8>::with_capacity(CAPACITY);
        let mut tex_size = 4096;
        let mut offsets = Vec::with_capacity(8);
        for mip in 0..8 {
            offsets.push(pixelbuf.len());
            pixelbuf.extend_from_slice(&atlas_data[mip].raw_pixels());
            tex_size /= 2;
        }

        loader_progress(0.20, "Feeding the video card", false);
        let buffer = context
            .device
            .create_buffer_with_data(&pixelbuf, wgpu::BufferUsage::COPY_SRC);

        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("texture_buffer_copy_encoder"),
            });

        tex_size = ATLAS_W;
        for mip in 0..8 {
            encoder.copy_buffer_to_texture(
                wgpu::BufferCopyView {
                    buffer: &buffer,
                    offset: offsets[mip as usize] as u64,
                    bytes_per_row: 4 * tex_size as u32,
                    rows_per_image: tex_size as u32,
                },
                wgpu::TextureCopyView {
                    texture: &texture,
                    mip_level: mip,
                    array_layer: 0,
                    origin: wgpu::Origin3d::ZERO,
                },
                wgpu::Extent3d {
                    width: tex_size as u32,
                    height: tex_size as u32,
                    depth: 1,
                },
            );
            tex_size /= 2;
        }

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
        Ok(Self {
            texture,
            view,
            sampler,
            dimensions: (ATLAS_W as usize, ATLAS_H as usize, 1),
            tex_map,
        })
    }
}

use image::*;

fn load_image_levels(base: &DynamicImage) -> [ImageBuffer<Rgba<u8>, Vec<u8>>; 8] {
    const TEX_FILTER: FilterType = FilterType::Lanczos3;
    const TS: u32 = TEXTURE_SIZE as u32;

    [
        base.to_rgba(),
        base.resize_exact(TS / 2, TS / 2, TEX_FILTER).to_rgba(),
        base.resize_exact(TS / 4, TS / 4, TEX_FILTER).to_rgba(),
        base.resize_exact(TS / 8, TS / 8, TEX_FILTER).to_rgba(),
        base.resize_exact(TS / 16, TS / 16, TEX_FILTER).to_rgba(),
        base.resize_exact(TS / 32, TS / 32, TEX_FILTER).to_rgba(),
        base.resize_exact(TS / 64, TS / 64, TEX_FILTER).to_rgba(),
        base.resize_exact(TS / 128, TS / 128, TEX_FILTER).to_rgba(),
    ]
}
