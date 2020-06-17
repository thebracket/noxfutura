use std::collections::HashMap;
use std::fs;

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
    pub fn blank(context: &crate::engine::Context) -> Result<Self, failure::Error> {
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
        let mut atlas_data = image::DynamicImage::new_rgba8(ATLAS_W, ATLAS_H);

        let mut matmap = HashMap::<String, usize>::new();
        for (i, (k, v)) in tex_map.iter().enumerate() {
            // i is index, k is stub-name, v is the index in the texture array and can be largely ignored?
            let albdeo_fn = format!("resources/terrain/{}-t.jpg", k);
            let normal_fn = format!("resources/terrain/{}-n.jpg", k);
            let rough_fn = format!("resources/terrain/{}-r.jpg", k);
            let ao_fn = format!("resources/terrain/{}-ao.jpg", k);
            let metal_fn = format!("resources/terrain/{}-m.jpg", k);

            matmap.insert(k.clone(), i * 3);

            //println!("Constructing {}: {}", i, k);
            let albdeo = image::open(&albdeo_fn).unwrap().into_rgba();
            let normal = image::open(&normal_fn).unwrap().into_rgba();
            let rough = image::open(&rough_fn).unwrap().into_rgba();
            let ao = image::open(&ao_fn).unwrap().into_rgba();
            let metal  = if std::path::Path::new(&metal_fn).exists() {
                image::open(&metal_fn).unwrap().into_rgba()
            } else {
                image::open("resources/metal-template.jpg").unwrap().into_rgba()
            };

            let base_i = i * 3;
            // Paste the albedo
            let x = (base_i % ATLAS_COLS) * TEXTURE_SIZE;
            let y = (base_i / ATLAS_COLS) * TEXTURE_SIZE;
            image::imageops::overlay(&mut atlas_data, &albdeo, x as u32, y as u32);

            // Paste the normal
            let x = ((base_i+1) % ATLAS_COLS) * TEXTURE_SIZE;
            let y = ((base_i+1) / ATLAS_COLS) * TEXTURE_SIZE;
            image::imageops::overlay(&mut atlas_data, &normal, x as u32, y as u32);

            let mut fancy = ao.clone();
            for iy in 0..TEXTURE_SIZE as u32 {
                for ix in 0..TEXTURE_SIZE as u32 {
                    let aop = ao.get_pixel(ix, iy);
                    let roughp = rough.get_pixel(ix, iy);
                    let metalp = metal.get_pixel(ix, iy);
                    let p = image::Rgba::<u8>([aop[0], roughp[0], metalp[0], 255]);
                    fancy.put_pixel(ix, iy, p);
                }
            }
            let x = ((base_i+2) % ATLAS_COLS) * TEXTURE_SIZE;
            let y = ((base_i+2) / ATLAS_COLS) * TEXTURE_SIZE;
            image::imageops::overlay(&mut atlas_data, &fancy, x as u32, y as u32);
        }

        {
            let mut rawlock = crate::raws::RAWS.write();
            let mats = rawlock.materials.materials.clone();
            rawlock.matmap.build(&matmap, &mats);
        }

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
            mip_level_count: 3,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        let mut pixelbuf = atlas_data.raw_pixels().clone();
        let mip1 = pixelbuf.len();
        pixelbuf.extend_from_slice(&atlas_data.clone().resize_exact(2048, 2048, image::FilterType::Gaussian).raw_pixels());
        let mip2 = pixelbuf.len();
        pixelbuf.extend_from_slice(&atlas_data.clone().resize_exact(1024, 1024, image::FilterType::Gaussian).raw_pixels());

        let size1 = wgpu::Extent3d {
            width: ATLAS_W as u32 / 2,
            height: ATLAS_H as u32 / 2,
            depth: 1,
        };
        let size2 = wgpu::Extent3d {
            width: ATLAS_W as u32 / 4,
            height: ATLAS_H as u32 / 4,
            depth: 1,
        };

        let buffer = context
            .device
            .create_buffer_with_data(&pixelbuf, wgpu::BufferUsage::COPY_SRC);

        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
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

        encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &buffer,
                offset: mip1 as u64,
                bytes_per_row: 4 * 2048,
                rows_per_image: 2048,
            },
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 1,
                array_layer: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            size1,
        );

        encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &buffer,
                offset: mip2 as u64,
                bytes_per_row: 4 * 1024,
                rows_per_image: 1024,
            },
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 2,
                array_layer: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            size2,
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
        Ok(Self {
            texture,
            view,
            sampler,
            dimensions: (ATLAS_W as usize, ATLAS_H as usize, 1),
            tex_map,
        })
    }
}
