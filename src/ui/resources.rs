use bevy::prelude::*;

pub struct UiResources {
    pub backgrounds: Handle<TextureAtlas>,
    pub worldgen_tex: Handle<Texture>,
    pub worldgen_seed: String,
}

pub struct BackgroundImage;

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("images/backgrounds.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(1280.0, 1024.0), 2, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let worldgen_tex = asset_server.load("images/worldgen_tiles.png");


    commands.insert_resource(UiResources {
        backgrounds: texture_atlas_handle.clone(),
        worldgen_seed: "Test Seed".to_string(),
        worldgen_tex,
    });

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(0),
            ..Default::default()
        })
        .insert(BackgroundImage {});
}
