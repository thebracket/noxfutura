use bevy::prelude::*;

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
) {
    crate::raws::load_raws();

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
        worldgen_tex,
        embark_tiles: embark_atlas_handle.clone(),
    });
}
