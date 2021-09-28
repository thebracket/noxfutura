use bevy::prelude::*;

pub struct UiResources {
    pub backgrounds: Handle<TextureAtlas>,
    pub worldgen_tex: Handle<Texture>,
    pub worldgen_seed: String,
    pub worldgen_lacunarity: f32,
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
    let texture_handle = asset_server.load("images/backgrounds.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(1280.0, 1024.0), 2, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let worldgen_tex = asset_server.load("images/worldgen_tiles.png");

    let embark_handle = asset_server.load("images/embark_tiles.png");
    let embark_atlas = TextureAtlas::from_grid(embark_handle, Vec2::new(8.0, 8.0), 8, 8);
    let embark_atlas_handle = texture_atlases.add(embark_atlas);

    commands.insert_resource(UiResources {
        backgrounds: texture_atlas_handle.clone(),
        worldgen_seed: "Test Seed".to_string(),
        worldgen_lacunarity: 2.0,
        worldgen_tex,
        embark_tiles: embark_atlas_handle.clone(),
    });

    let tree_tex: Handle<Texture> = asset_server.load("obj/treeTall.png");
    let tree_mat = materials.add(StandardMaterial {
        base_color_texture: Some(tree_tex),
        roughness: 0.8,
        ..Default::default()
    });
    crate::simulation::terrain::PLANET_STORE.write().tree_mat = Some(tree_mat);

    let vox_tex: Handle<Texture> = asset_server.load("vox/palette.png");
    let vox_mat = materials.add(StandardMaterial {
        base_color_texture: Some(vox_tex),
        roughness: 0.8,
        ..Default::default()
    });
    crate::simulation::terrain::PLANET_STORE.write().vox_mat = Some(vox_mat);
}
