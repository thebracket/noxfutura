use bevy::{app::Events, prelude::*};
use bevy_egui::{egui, EguiContext};

use crate::AppState;

pub struct MainMenuBackground;

pub fn main_menu(
    egui_context: ResMut<EguiContext>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut state: ResMut<State<AppState>>,
) {
    egui::Window::new("Hello")
        .title_bar(false)
        .show(egui_context.ctx(), |ui| {
            if ui.button("Create World").clicked() {
                state.set(AppState::WorldGenMenu).unwrap();
            }

            // Quit game option
            if ui.button("Quit").clicked() {
                app_exit_events.send(bevy::app::AppExit);
            }
        });
}

pub fn main_menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let background_image = asset_server.load("images/background_image.png");
    let background_handle = materials.add(background_image.into());
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        material: background_handle,
        ..Default::default()
    }).insert(MainMenuBackground{});
}

pub fn main_menu_cleanup(
    query: Query<(Entity, &MainMenuBackground)>,
    mut commands: Commands,
) {
    for (e, _) in query.iter() {
        commands.entity(e).despawn();
    }
}