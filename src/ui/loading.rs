use crate::AppState;
use bevy::{app::Events, prelude::*, render::texture::AddressMode};
use bevy_egui::{
    egui::{self, Pos2},
    EguiContext,
};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

pub struct LoadingResource {
    cycle: u8,
    world_textures: HashSet<Handle<StandardMaterial>>,
    total_textures: usize,
}

pub fn loading_screen(
    ev_asset: Res<Events<AssetEvent<StandardMaterial>>>,
    egui_context: ResMut<EguiContext>,
    state: ResMut<State<AppState>>,
    mut res: ResMut<LoadingResource>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    assets: ResMut<Assets<Texture>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    egui::Window::new("Loading - Please Wait")
        .auto_sized()
        .resizable(false)
        .title_bar(false)
        .fixed_pos(Pos2::new(500.0, 200.0))
        .show(egui_context.ctx(), |ui| match res.cycle {
            0..=2 => res.cycle += 1,
            3 => load_raws(&mut res, ui),
            4 => load_textures(&mut res, &asset_server, &mut materials, ui, &mut meshes),
            5 => texture_events(ui, res, ev_asset, materials, assets, state),
            _ => {}
        });
}

fn texture_events(
    ui: &mut egui::Ui,
    mut res: ResMut<LoadingResource>,
    ev_asset: Res<Events<AssetEvent<StandardMaterial>>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut assets: ResMut<Assets<Texture>>,
    mut state: ResMut<State<AppState>>,
) {
    ui.label(format!(
        "Material {} of {}",
        res.world_textures.len(),
        res.total_textures
    ));
    let mut evr_asset = ev_asset.get_reader();
    for event in evr_asset.iter(&ev_asset) {
        if let AssetEvent::Created { handle } = event {
            if make_texture_repeat(handle, &mut materials, &mut assets) {
                res.world_textures.remove(&handle);
            }
        }
    }
    if res.world_textures.is_empty() {
        state
            .set(AppState::MainMenu)
            .expect("Failed to change mode");
    }
}

fn load_textures(
    res: &mut LoadingResource,
    asset_server: &AssetServer,
    materials: &mut Assets<StandardMaterial>,
    ui: &mut egui::Ui,
    meshes: &mut Assets<Mesh>,
) {
    res.cycle += 1;
    ui.label("Loading World Textures");

    let mut tex_map = HashMap::new();
    let raw_read = crate::raws::RAWS.read();
    raw_read.materials.materials.iter().for_each(|m| {
        if let Some(texture) = &m.texture {
            if let Some(texture_name) = &texture.base {
                if !tex_map.contains_key(texture_name) {
                    if let Some(tex_handle) = load_image_if_exists(texture_name, &asset_server) {
                        tex_map.insert(texture_name.clone(), tex_handle.clone());
                    }
                    if let Some(tex_handle) =
                        load_image_if_exists(&format!("{}-n", texture_name), &asset_server)
                    {
                        tex_map.insert(format!("{}-n", texture_name), tex_handle.clone());
                    }
                }
            }
        }
    });
    let mut matmap = Vec::new();
    raw_read.materials.materials.iter().for_each(|m| {
        let mut fancy = false;
        if let Some(texture) = &m.texture {
            if let Some(texture_name) = &texture.base {
                if let Some(th) = tex_map.get(texture_name) {
                    fancy = true;

                    let world_material_handle = materials.add(StandardMaterial {
                        base_color_texture: Some(th.clone()),
                        normal_map: if let Some(nm) = tex_map.get(&format!("{}-n", texture_name)) {
                            Some(nm.clone())
                        } else {
                            None
                        },
                        roughness: texture.roughness.unwrap_or(0.8),
                        unlit: false,
                        ..Default::default()
                    });
                    matmap.push(world_material_handle.clone());
                    res.world_textures.insert(world_material_handle.clone());
                }
            }
        }

        if !fancy {
            let world_material_handle = materials.add(StandardMaterial {
                base_color: Color::rgb(m.tint.0, m.tint.1, m.tint.2),
                roughness: 0.5,
                unlit: false,
                ..Default::default()
            });
            matmap.push(world_material_handle);
        }
    });
    res.total_textures = res.world_textures.len();
    crate::simulation::terrain::PLANET_STORE
        .write()
        .world_material_handle = Some(matmap);

    // Load grass
    let grass_tex_handle = load_image_if_exists("grass", &asset_server).unwrap();
    let grass_normal_handle = load_image_if_exists("grass-n", &asset_server).unwrap();
    let grass_handle = materials.add(StandardMaterial {
        base_color_texture: Some(grass_tex_handle),
        normal_map: Some(grass_normal_handle),
        roughness: 0.5,
        unlit: false,
        ..Default::default()
    });
    crate::simulation::terrain::PLANET_STORE
        .write()
        .grass_handle = Some(grass_handle.clone());

    // Load wavefront - test code
    let tree_handle: Handle<Mesh> = asset_server.load("obj/Low_Poly_Forest_treeTall01.obj");
    crate::simulation::terrain::PLANET_STORE.write().tree_handle = Some(tree_handle.clone());

    // Load all voxel meshes
    for vox in crate::raws::RAWS.read().vox.vox.iter() {
        let filename = format!("assets/vox/{}.vox", vox.file);
        let template = crate::asset_handlers::vox::load_vox_file(&filename);
        crate::simulation::terrain::PLANET_STORE.write().vox_templates.push(template.clone());
        crate::simulation::terrain::PLANET_STORE.write().vox_meshes.push(meshes.add(template.to_mesh()));
    }
}

fn load_raws(res: &mut ResMut<LoadingResource>, ui: &mut egui::Ui) {
    res.cycle += 1;
    ui.label("Loading Raw Files");
    crate::raws::load_raws();
}

fn load_image_if_exists(texture_name: &str, asset_server: &AssetServer) -> Option<Handle<Texture>> {
    let filename = format!("assets/terrain/{}.png", texture_name);
    println!("{}", filename);
    let filename_bevy = format!("terrain/{}.png", texture_name);
    let path = Path::new(&filename);
    if path.exists() {
        Some(asset_server.load(filename_bevy.as_str()))
    } else {
        None
    }
}

fn make_texture_repeat(
    handle: &Handle<StandardMaterial>,
    materials: &mut Assets<StandardMaterial>,
    assets: &mut Assets<Texture>,
) -> bool {
    let mut result = false;
    if let Some(mat) = materials.get_mut(handle) {
        if let Some(th) = &mat.base_color_texture {
            if let Some(t) = assets.get_mut(th) {
                println!("Fixed up texture");
                t.sampler.address_mode_u = AddressMode::Repeat;
                t.sampler.address_mode_v = AddressMode::Repeat;
                t.sampler.address_mode_w = AddressMode::Repeat;
                result = true;
            }
        }
        if let Some(th) = &mat.normal_map {
            println!("Found a normal map");
            if let Some(t) = assets.get_mut(th) {
                println!("Fixed up normal map");
                t.sampler.address_mode_u = AddressMode::Repeat;
                t.sampler.address_mode_v = AddressMode::Repeat;
                t.sampler.address_mode_w = AddressMode::Repeat;
                result = true;
            }
        }
    }
    result
}

pub fn resume_loading_screen(mut commands: Commands) {
    commands.insert_resource(LoadingResource {
        cycle: 0,
        world_textures: HashSet::new(),
        total_textures: 0,
    });
}

pub fn exit_loading(mut commands: Commands) {
    commands.remove_resource::<LoadingResource>();
}
