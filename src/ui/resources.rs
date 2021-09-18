use bevy::prelude::*;
use std::{collections::HashMap, path::Path};

pub struct UiResources {
    pub backgrounds: Handle<TextureAtlas>,
    pub worldgen_tex: Handle<Texture>,
    pub worldgen_seed: String,
    pub embark_tiles: Handle<TextureAtlas>,
}

pub struct BackgroundImage;
pub struct UiCamera;

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    crate::raws::load_raws();
    crate::simulation::terrain::CHUNK_STORE
        .write()
        .verify_strata();

    let texture_handle = asset_server.load("images/backgrounds.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(1280.0, 1024.0), 2, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let worldgen_tex = asset_server.load("images/worldgen_tiles.png");

    let embark_handle = asset_server.load("images/embark_tiles.png");
    let embark_atlas = TextureAtlas::from_grid(embark_handle, Vec2::new(8.0, 8.0), 8, 8);
    let embark_atlas_handle = texture_atlases.add(embark_atlas);

    // World materials
    let mut tex_map = HashMap::new();
    let raw_read = crate::raws::RAWS.read();
    raw_read.materials.materials.iter().for_each(|m| {
        if let Some(texture) = &m.texture {
            if let Some(texture_name) = &texture.base {
                if !tex_map.contains_key(texture_name) {
                    let filename = format!("assets/terrain/{}.png", texture_name);
                    let filename_bevy = format!("terrain/{}.png", texture_name);
                    let path = Path::new(&filename);
                    if path.exists() {
                        let tex_handle: Handle<Texture> = asset_server.load(filename_bevy.as_str());
                        tex_map.insert(texture_name.clone(), tex_handle);
                    }
                }
            }
        }
    });
    println!("{:#?}", tex_map);

    let mut matmap = Vec::new();
    raw_read
        .materials
        .materials
        .iter()
        .for_each(|m| {
            let mut fancy = false;
            if let Some(texture) = &m.texture {
                if let Some(texture_name) = &texture.base {
                    if let Some(th) = tex_map.get(texture_name) {
                        fancy = true;

                        let world_material_handle = materials.add(StandardMaterial {
                            base_color_texture: Some(th.clone()),
                            roughness: 0.5,
                            unlit: false,
                            ..Default::default()
                        });
                        matmap.push(world_material_handle);
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

    crate::simulation::terrain::PLANET_STORE
        .write()
        .world_material_handle = Some(matmap);

    commands.insert_resource(UiResources {
        backgrounds: texture_atlas_handle.clone(),
        worldgen_seed: "Test Seed".to_string(),
        worldgen_tex,
        embark_tiles: embark_atlas_handle.clone(),
    });
}
