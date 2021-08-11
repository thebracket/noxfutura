use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub struct WorldgenMenuBackground;

pub fn world_gen_menu(
    egui_context: ResMut<EguiContext>,
) {
    egui::Window::new("Hello")
        .title_bar(false)
        .show(egui_context.ctx(), |ui| {
            ui.label("Goes Here");
        });
}

pub fn world_gen_menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let background_image = asset_server.load("images/starscape.png");
    let background_handle = materials.add(background_image.into());
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        material: background_handle,
        ..Default::default()
    }).insert(WorldgenMenuBackground{});
}

pub fn world_gen_menu_cleanup(
    query: Query<(Entity, &WorldgenMenuBackground)>,
    mut commands: Commands,
) {
    for (e, _) in query.iter() {
        commands.entity(e).despawn();
    }
}