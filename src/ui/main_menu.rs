use bevy::{app::Events, prelude::*};
use bevy_egui::{
    egui::{self, Color32, Pos2},
    EguiContext,
};
use bracket_random::prelude::RandomNumberGenerator;

use crate::AppState;

use super::{BackgroundImage, UiCamera, UiResources};

pub struct MainMenuState {
    tagline: String,
}

pub fn main_menu(
    egui_context: ResMut<EguiContext>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut state: ResMut<State<AppState>>,
    mms: Res<MainMenuState>,
) {
    egui::Window::new("Hello")
        .auto_sized()
        .resizable(false)
        .title_bar(false)
        .fixed_pos(Pos2::new(500.0, 200.0))
        .show(egui_context.ctx(), |ui| {
            ui.colored_label(Color32::from_rgb(255, 0, 0), &mms.tagline);

            if ui.button("Create World").clicked() {
                state.set(AppState::WorldGenMenu).unwrap();
            }

            if std::path::Path::new("savegame/world.dat").exists() {
                if ui.button("Embark").clicked() {}
            }

            // Quit game option
            if ui.button("Quit").clicked() {
                app_exit_events.send(bevy::app::AppExit);
            }
        });

    egui::Window::new("Dedication")
        .auto_sized()
        .resizable(false)
        .title_bar(false)
        .fixed_pos(Pos2::new(400.0, 100.0))
        .show(egui_context.ctx(), |ui| {
            ui.colored_label(Color32::from_rgb(255, 255, 255), DEDICATION);
        });

    egui::Window::new("Copyright")
        .title_bar(false)
        .auto_sized()
        .resizable(false)
        .fixed_pos(Pos2::new(500.0, 1024.0 - 30.0))
        .show(egui_context.ctx(), |ui| {
            ui.colored_label(Color32::from_rgb(255, 255, 0), COPYRIGHT);
        });
}

pub fn resume_main_menu(mut commands: Commands, ui: Res<UiResources>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(UiCamera {});
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: ui.backgrounds.clone(),
            sprite: TextureAtlasSprite::new(0),
            ..Default::default()
        })
        .insert(BackgroundImage {});
}

pub fn setup_main_menu(mut commands: Commands) {
    commands.insert_resource(MainMenuState { tagline: tagline() });
}

pub fn exit_main_menu(
    mut commands: Commands,
    q: Query<(Entity, &UiCamera)>,
    q2: Query<(Entity, &BackgroundImage)>,
) {
    q.iter()
        .for_each(|(entity, _)| commands.entity(entity).despawn());
    q2.iter()
        .for_each(|(entity, _)| commands.entity(entity).despawn());
}

fn tagline() -> String {
    let mut rng = RandomNumberGenerator::new();
    let mut tagline = match rng.roll_dice(1, 8) {
        1 => "Histories",
        2 => "Chronicles",
        3 => "Sagas",
        4 => "Annals",
        5 => "Narratives",
        6 => "Recitals",
        7 => "Tales",
        8 => "Stories",
        _ => "",
    }
    .into();

    let first_noun = get_descriptive_noun(&mut rng);
    let mut second_noun = get_descriptive_noun(&mut rng);
    while first_noun == second_noun {
        second_noun = get_descriptive_noun(&mut rng);
    }

    tagline = format!("{} of {} and {}", tagline, first_noun, second_noun).to_string();

    tagline
}

fn get_descriptive_noun(rng: &mut RandomNumberGenerator) -> String {
    rng.random_slice_entry(&NOUNS).unwrap().to_string()
}

const NOUNS: &'static [&'static str] = &[
    "Stupidity",
    "Idiocy",
    "Dullness",
    "Foolishness",
    "Futility",
    "Naievity",
    "Senselessness",
    "Shortsightedness",
    "Triviality",
    "Brainlessness",
    "Inanity",
    "Insensitivity",
    "Indiscretion",
    "Mindlessness",
    "Moronism",
    "Myopia",
    "Obtuseness",
    "Obliviousness",
    "Unthinkingness",
];

const DEDICATION: &str =
    "To Kylah of the West and Jakie Monster: The Bravest Little Warriors of Them All.";
const COPYRIGHT: &str = "(c) 2015-2020 Bracket Productions, All Rights Reserved.";
