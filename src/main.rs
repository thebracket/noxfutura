use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_simple_tilemap::prelude::*;
mod ui;
use simulation::terrain::{game_camera_system, manage_terrain_tasks};
use ui::*;
mod geometry;
mod raws;
mod simulation;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AppState {
    Loading,
    MainMenu,
    WorldGenMenu,
    BuildingPlanet,
    Embark,
    EmbarkBuildRegion,
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Nox Futura".to_string(),
            width: 1280.0,
            height: 1024.0,
            vsync: false,
            resizable: false,
            ..Default::default()
        })
        .add_state(AppState::Loading)
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(SimpleTileMapPlugin)
        .add_startup_system(setup_fps.system())
        .add_startup_system(setup_ui.system())
        .add_startup_system(setup_main_menu.system())
        .add_system(fps_update_system.system())
        // Loading State
        .add_system_set(
            SystemSet::on_update(AppState::Loading)
            .with_system(loading_screen.system())
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Loading)
            .with_system(resume_loading_screen.system())
        )
        .add_system_set(
            SystemSet::on_exit(AppState::Loading)
            .with_system(exit_loading.system())
        )
        // Main Menu State
        .add_system_set(
            SystemSet::on_update(AppState::MainMenu)
            .with_system(main_menu.system())
            //.with_system(texture_mode_system.system())
        )
        .add_system_set(
            SystemSet::on_enter(AppState::MainMenu)
            .with_system(resume_main_menu.system())
        )
        .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(exit_main_menu.system()))
        // World-gen Menu
        .add_system_set(
            SystemSet::on_update(AppState::WorldGenMenu)
                .with_system(world_gen_menu.system())
        )
        .add_system_set(
            SystemSet::on_enter(AppState::WorldGenMenu).with_system(resume_world_gen_menu.system()),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::WorldGenMenu).with_system(world_gen_exit.system()),
        )
        // Planet Builder Menu
        .add_system_set(
            SystemSet::on_update(AppState::BuildingPlanet)
                .with_system(planet_builder_menu.system()),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::BuildingPlanet)
                .with_system(resume_planet_builder_menu.system())
                .label("PlanetBuilderResume"),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::BuildingPlanet).with_system(planet_builder_exit.system()),
        )
        // Embark Menu
        .add_system_set(SystemSet::on_update(AppState::Embark).with_system(embark_menu.system()))
        .add_system_set(
            SystemSet::on_enter(AppState::Embark).with_system(resume_embark_menu.system()),
        )
        .add_system_set(SystemSet::on_exit(AppState::Embark).with_system(embark_exit.system()))
        // Embark Region Menu
        .add_system_set(
            SystemSet::on_update(AppState::EmbarkBuildRegion)
                .with_system(embark_region_menu.system())
                .with_system(game_camera_system.system())
                .with_system(manage_terrain_tasks.system()),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::EmbarkBuildRegion)
                .with_system(resume_embark_region.system()),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::EmbarkBuildRegion)
                .with_system(embark_region_exit.system()),
        )
        // Start
        .run();
}
