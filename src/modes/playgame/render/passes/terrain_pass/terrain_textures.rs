use crate::raws::{TextureMap, RAWS};
use bengine::*;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::Read;

pub struct TerrainTextures {
    pub bind_group_layout: gpu::BindGroupLayout,
    pub bind_group: gpu::BindGroup,
}

impl TerrainTextures {
    pub fn new() -> Self {
        let mut texture_set = HashSet::new();
        let material_list_tmp = build_material_list(&mut texture_set);
        let texture_list = texture_set
            .iter()
            .map(|s| s.clone())
            .collect::<Vec<String>>();
        let material_list = build_texture_list(&material_list_tmp, &texture_list);
        let mut tex_views = Vec::new();
        texture_list.iter().for_each(|base_fn| {
            let texture_filename = format!("resources/terrain/{}.png", base_fn);
            tex_views.push(build_texture_view(&texture_filename));
        });
        println!("{:#?}", texture_list);

        let rcl = RENDER_CONTEXT.read();
        let rc = rcl.as_ref().unwrap();

        let sampler = rc.device.create_sampler(&gpu::SamplerDescriptor {
            address_mode_u: gpu::AddressMode::Repeat,
            address_mode_v: gpu::AddressMode::Repeat,
            ..gpu::SamplerDescriptor::default()
        });

        println!("Binding layout for {} textures", texture_list.len() as u32);
        let bind_group_layout =
            rc.device
                .create_bind_group_layout(&gpu::BindGroupLayoutDescriptor {
                    label: Some("tabgl"),
                    entries: &[
                        gpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: gpu::ShaderStage::FRAGMENT,
                            ty: gpu::BindingType::SampledTexture {
                                component_type: gpu::TextureComponentType::Uint,
                                dimension: gpu::TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: std::num::NonZeroU32::new(texture_list.len() as u32),
                        },
                        gpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: gpu::ShaderStage::FRAGMENT,
                            ty: gpu::BindingType::Sampler { comparison: false },
                            count: None,
                        },
                    ],
                });

        let bind_group = rc.device.create_bind_group(&gpu::BindGroupDescriptor {
            entries: &[
                gpu::BindGroupEntry {
                    binding: 0,
                    resource: gpu::BindingResource::TextureViewArray(&tex_views),
                },
                gpu::BindGroupEntry {
                    binding: 1,
                    resource: gpu::BindingResource::Sampler(&sampler),
                },
            ],
            layout: &bind_group_layout,
            label: Some("texture_array"),
        });

        let mut rlock = RAWS.write();
        rlock.materials.material_list = Some(material_list);

        Self {
            bind_group_layout,
            bind_group,
        }
    }
}

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn build_material_list(
    texture_set: &mut HashSet<String>,
) -> Vec<(
    usize,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
)> {
    let mut material_list_tmp = Vec::new();

    let rlock = RAWS.read();
    // Read the material index and create a de-duplicated list of textures to load
    rlock
        .materials
        .materials
        .iter()
        .enumerate()
        .for_each(|(i, m)| {
            let mut mat_map_tmp = (i, None, None, None, None);
            if let Some(texture_base) = &m.texture {
                if let Some(base) = &texture_base.base {
                    texture_set.insert(base.clone());
                    mat_map_tmp.1 = Some(base.clone());
                }
                if let Some(base) = &texture_base.constructed {
                    texture_set.insert(base.clone());
                    mat_map_tmp.2 = Some(base.clone());
                }
                if let Some(floor) = &texture_base.floor {
                    texture_set.insert(floor.clone());
                    mat_map_tmp.3 = Some(floor.clone());
                }
                if let Some(base) = &texture_base.floor_constructed {
                    texture_set.insert(base.clone());
                    mat_map_tmp.4 = Some(base.clone());
                }
            }
            material_list_tmp.push(mat_map_tmp);
        });

    material_list_tmp
}

fn build_texture_list(
    material_list_tmp: &Vec<(
        usize,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
    )>,
    texture_list: &Vec<String>,
) -> Vec<TextureMap> {
    material_list_tmp
        .iter()
        .map(|m| {
            (
                m.0,
                if let Some(t) = &m.1 {
                    texture_list
                        .iter()
                        .enumerate()
                        .find(|(_i, tex)| **tex == *t)
                        .unwrap()
                        .0 as f32
                } else {
                    -1.0
                },
                if let Some(t) = &m.2 {
                    texture_list
                        .iter()
                        .enumerate()
                        .find(|(_i, tex)| **tex == *t)
                        .unwrap()
                        .0 as f32
                } else {
                    -1.0
                },
                if let Some(t) = &m.3 {
                    texture_list
                        .iter()
                        .enumerate()
                        .find(|(_i, tex)| **tex == *t)
                        .unwrap()
                        .0 as f32
                } else {
                    -1.0
                },
                if let Some(t) = &m.4 {
                    texture_list
                        .iter()
                        .enumerate()
                        .find(|(_i, tex)| **tex == *t)
                        .unwrap()
                        .0 as f32
                } else {
                    -1.0
                },
            )
        })
        .map(|tmp| TextureMap {
            id: tmp.0,
            base: tmp.1,
            constructed: tmp.2,
            floor: tmp.3,
            floor_constructed: tmp.4,
        })
        .collect()
}

fn build_texture_view(filename: &str) -> gpu::TextureView {
    println!("Loading texture: [{}]", filename);
    use image::GenericImageView;

    let bytes = get_file_as_byte_vec(&filename.to_string());
    let img = image::load_from_memory(&bytes).expect("Unable to read bytes");
    let rgba = img.as_rgba8().expect("Unable to get RGBA8 buffer");
    let dimensions = img.dimensions();

    let rcl = RENDER_CONTEXT.read();
    let rc = rcl.as_ref().unwrap();

    let size = gpu::Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        depth: 1,
    };
    let texture = rc.device.create_texture(&gpu::TextureDescriptor {
        label: Some(filename),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: gpu::TextureDimension::D2,
        format: gpu::TextureFormat::Rgba8UnormSrgb,
        usage: gpu::TextureUsage::SAMPLED | gpu::TextureUsage::COPY_DST,
    });

    rc.queue.write_texture(
        gpu::TextureCopyView {
            texture: &texture,
            mip_level: 0,
            origin: gpu::Origin3d::ZERO,
        },
        &rgba,
        gpu::TextureDataLayout {
            offset: 0,
            bytes_per_row: 4 * size.width,
            rows_per_image: size.height,
        },
        size,
    );

    texture.create_view(&gpu::TextureViewDescriptor::default())
}
