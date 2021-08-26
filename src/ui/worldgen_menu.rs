use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Pos2},
    EguiContext,
};

use crate::AppState;

use super::{BackgroundImage, UiResources};

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

            if ui.button("Create World").clicked() {
                state.set(AppState::BuildingPlanet).unwrap();
            }

            if ui.button("Return to Main Menu").clicked() {
                state.set(AppState::MainMenu).unwrap();
            }
        });
}

pub fn resume_world_gen_menu(mut query: Query<(&mut TextureAtlasSprite, &BackgroundImage)>) {
    for (mut sprite, _) in query.iter_mut() {
        sprite.index = 1;
    }
}
