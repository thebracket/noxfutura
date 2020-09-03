use crate::modes::loader_progress;
use bengine::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;

const TEXTURE_SIZE: usize = 256;
const ATLAS_COLS: usize = 16;
const ATLAS_W: u32 = (ATLAS_COLS * TEXTURE_SIZE) as u32;
const ATLAS_H: u32 = (ATLAS_COLS * TEXTURE_SIZE) as u32;

pub struct TextureArray {
    pub texture: gpu::Texture,
    pub view: gpu::TextureView,
    pub sampler: gpu::Sampler,
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
        let mut progress = 0.17;
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

            let (albedo_mip, normal_mip) =
                rayon::join(|| load_image_levels(&albedo), || load_image_levels(&normal));
            let (rough_mip, ao_mip) =
                rayon::join(|| load_image_levels(&rough), || load_image_levels(&ao));

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

            loader_progress(progress, &format!("Squished {}", k), false);
            progress += 0.01;
        }

        loader_progress(0.25, "Mapping Materials", false);
        {
            let mut rawlock = crate::RAWS.write();
            let mats = rawlock.materials.materials.clone();
            rawlock.matmap.build(&matmap, &mats);
        }

        // Build the texture
        loader_progress(0.26, "Throwing data around", false);
        let size = gpu::Extent3d {
            width: ATLAS_W as u32,
            height: ATLAS_H as u32,
            depth: 1,
        };
        let dl = RENDER_CONTEXT.read();
        let context = dl.as_ref().unwrap();
        let texture = context.device.create_texture(&gpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 8,
            sample_count: 1,
            dimension: gpu::TextureDimension::D2,
            format: gpu::TextureFormat::Rgba8Unorm,
            usage: gpu::TextureUsage::SAMPLED | gpu::TextureUsage::COPY_DST,
        });

        loader_progress(0.27, "Feeding the video card", false);

        let mut tex_size = ATLAS_W;
        let mut size = size;
        for mip in 0..8 {
            context.queue.write_texture(
                gpu::TextureCopyView {
                    texture: &texture,
                    mip_level: mip as u32,
                    origin: gpu::Origin3d::ZERO,
                },
                &atlas_data[mip].to_bytes(),
                gpu::TextureDataLayout {
                    offset: 0,
                    bytes_per_row: 4 * tex_size as u32,
                    rows_per_image: 0,
                },
                size,
            );
            size.width /= 2;
            size.height /= 2;
            tex_size /= 2;
        }

        let view = texture.create_view(&gpu::TextureViewDescriptor::default());
        let sampler = context.device.create_sampler(&gpu::SamplerDescriptor {
            label: None,
            anisotropy_clamp: None,
            address_mode_u: gpu::AddressMode::Repeat,
            address_mode_v: gpu::AddressMode::Repeat,
            address_mode_w: gpu::AddressMode::Repeat,
            mag_filter: gpu::FilterMode::Nearest,
            min_filter: gpu::FilterMode::Nearest,
            mipmap_filter: gpu::FilterMode::Linear,
            lod_min_clamp: 0.0,
            lod_max_clamp: 7.0,
            compare: Some(gpu::CompareFunction::Always),
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

fn load_image_levels(base: &DynamicImage) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    const TEX_FILTER: imageops::FilterType = imageops::FilterType::Triangle;
    const TS: u32 = TEXTURE_SIZE as u32;

    let mut result_optional: [(usize, Option<ImageBuffer<Rgba<u8>, Vec<u8>>>); 8] = [
        (0, None),
        (1, None),
        (2, None),
        (3, None),
        (4, None),
        (5, None),
        (6, None),
        (7, None),
    ];
    result_optional.par_iter_mut().for_each(|(i, r)| {
        let sz = TS / (1 << *i as u32);
        *r = Some(base.resize_exact(sz, sz, TEX_FILTER).to_rgba());
    });
    result_optional
        .iter()
        .map(|(_, r)| r.as_ref().unwrap().clone())
        .collect::<Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>>()
}
