use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Pos2},
    EguiContext,
};

use crate::AppState;

use super::{BackgroundImage, UiCamera, UiResources};

pub fn world_gen_menu(
    egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<AppState>>,
    mut res: ResMut<UiResources>,
) {
    egui::Window::new("Hello")
        .title_bar(false)
        .fixed_pos(Pos2::new(100.0, 100.0))
        .show(egui_context.ctx(), |ui| {
            ui.label("Random Seed");
            ui.text_edit_singleline(&mut res.worldgen_seed);

            ui.label("Bumpiness");
            ui.add(
                egui::Slider::new(&mut res.worldgen_lacunarity, 2.0 ..= 4.0).clamp_to_range(true)
            );

            if ui.button("Create World").clicked() {
                state.set(AppState::BuildingPlanet).unwrap();
            }

            if ui.button("Return to Main Menu").clicked() {
                state.set(AppState::MainMenu).unwrap();
            }
        });
}

pub fn resume_world_gen_menu(mut commands: Commands, ui: Res<UiResources>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(UiCamera {});
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: ui.backgrounds.clone(),
            sprite: TextureAtlasSprite::new(1),
            ..Default::default()
        })
        .insert(BackgroundImage {});
}

pub fn world_gen_exit(
    mut commands: Commands,
    q: Query<(Entity, &UiCamera)>,
    q2: Query<(Entity, &BackgroundImage)>,
) {
    q.iter()
        .for_each(|(entity, _)| commands.entity(entity).despawn());
    q2.iter()
        .for_each(|(entity, _)| commands.entity(entity).despawn());
}
