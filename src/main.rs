use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_egui::EguiPlugin;
mod ui;
use ui::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AppState {
    MainMenu,
    WorldGenMenu,
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Nox Futura".to_string(),
            width: 1280.0,
            height: 1024.0,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_state(AppState::MainMenu)
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup_fps.system())
        .add_system(fps_update_system.system())

        // Main Menu State
        .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(main_menu_setup.system()))
        .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(main_menu.system()))
        .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(main_menu_cleanup.system()))

        // World-gen Menu
        .add_system_set(SystemSet::on_enter(AppState::WorldGenMenu).with_system(world_gen_menu_setup.system()))
        .add_system_set(SystemSet::on_update(AppState::WorldGenMenu).with_system(world_gen_menu.system()))
        .add_system_set(SystemSet::on_exit(AppState::WorldGenMenu).with_system(world_gen_menu_cleanup.system()))

        // Start
        .run();
}
