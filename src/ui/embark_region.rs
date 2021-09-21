use bevy::{pbr::AmbientLight, prelude::*, tasks::AsyncComputeTaskPool};
use bevy_egui::{
    egui::{self, Pos2},
    EguiContext,
};
use crate::simulation::{region_builder::RegionBuilder, terrain::spawn_game_camera};
use super::EmbarkResources;

pub struct RegionGenUi;

pub fn embark_region_menu(
    egui_context: ResMut<EguiContext>,
    mut builder: ResMut<RegionBuilder>,
    mut commands: Commands,
    task_master: Res<AsyncComputeTaskPool>,
    mut ambient: ResMut<AmbientLight>,
) {
    builder.start(task_master.clone(), &mut commands);

    egui::Window::new("Building Embark Region")
        .title_bar(true)
        .fixed_pos(Pos2::new(10.0, 10.0))
        .show(egui_context.ctx(), |ui| {
            ui.label(builder.status());
        });

    ambient.color = Color::WHITE;
    ambient.brightness = 0.3;
}

pub fn resume_embark_region(
    mut commands: Commands,
    embark: Res<EmbarkResources>,
    asset_server: Res<AssetServer>,
) {
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 12.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 12.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(crate::ui::fps::FpsText);

    // Region build link
    spawn_game_camera(&mut commands, embark.tile_x, embark.tile_y, 128, 128, 200);
    let rb = RegionBuilder::new(embark.planet.clone(), embark.tile_x, embark.tile_y);
    commands.insert_resource(rb);
}

pub fn embark_region_exit(mut commands: Commands, q: Query<(Entity, &RegionGenUi)>) {
    q.iter().for_each(|(e, _)| commands.entity(e).despawn());
}
