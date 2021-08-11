use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Pos2},
    EguiContext,
};

use crate::AppState;

use super::BackgroundImage;

pub fn world_gen_menu(egui_context: ResMut<EguiContext>, mut state: ResMut<State<AppState>>) {
    egui::Window::new("Hello")
        .title_bar(false)
        .fixed_pos(Pos2::new(100.0, 100.0))
        .show(egui_context.ctx(), |ui| {
            ui.label("Goes Here");
            if ui.button("Return to Main Menu").clicked() {
                state.set(AppState::MainMenu).unwrap();
            }
        });
}

pub fn resume_world_gen_menu(mut query: Query<(&mut TextureAtlasSprite, &BackgroundImage)>) {
    println!("Resume WG");
    for (mut sprite, _) in query.iter_mut() {
        //sb.material = images.nox_image.clone();
        sprite.index = 1;
    }
}
