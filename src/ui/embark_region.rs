use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Pos2},
    EguiContext,
};

use crate::simulation::{region_builder::RegionBuilder, REGION_HEIGHT, REGION_WIDTH};

use super::EmbarkResources;

pub struct RegionGenUi;

pub fn embark_region_menu(egui_context: ResMut<EguiContext>
    , mut builder: ResMut<RegionBuilder>, 
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
)
{
    builder.start();

    egui::Window::new("Building Embark Region")
        .title_bar(true)
        .fixed_pos(Pos2::new(10.0, 10.0))
        .show(egui_context.ctx(), |ui| {
            ui.label(builder.status());
        });

    if let Some(mut chunks) = builder.chunks() {
        let material_handle = materials.add(StandardMaterial {
            base_color: Color::rgb(0.0, 1.0, 0.0),
            roughness: 0.5,
            unlit: false,
            ..Default::default()
        });
        while !chunks.is_empty() {
            let c = chunks.pop().unwrap();

            // Insert the mesh as an asset
            let mesh_handle = meshes.add(c);

            commands
            .spawn_bundle(PbrBundle {
                mesh: mesh_handle.clone(),
                material: material_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            });
        }

        // light
        commands.spawn_bundle(LightBundle {
            transform: Transform::from_xyz(0.0, 2.0, 158.0),
            light: Light{ color: Color::rgb(1.0, 1.0, 1.0), fov: 90.0, depth: -256.0..256.0, range: 256.0, intensity: 5000.0 },
            ..Default::default()
        });
        // camera
        commands.spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 2.0, 158.0).looking_at(Vec3::new(256.0, 256.0, 120.0), Vec3::Z),
            ..Default::default()
        });
    }
}

pub fn resume_embark_region(mut commands: Commands, embark: Res<EmbarkResources>, asset_server: Res<AssetServer>) {
    // Camera
    /*commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(
                (REGION_WIDTH as f32 * 1.25) + 2.0,
                REGION_HEIGHT as f32 * 1.25,
                256.0,
            )
            .looking_at(Vec3::ZERO, Vec3::Z),
            ..Default::default()
        })
        .insert(RegionGenUi {});*/

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
    let rb = RegionBuilder::new(embark.planet.clone(), embark.tile_x, embark.tile_y);
    commands.insert_resource(rb);
}

pub fn embark_region_exit(mut commands: Commands, q: Query<(Entity, &RegionGenUi)>) {
    q.iter().for_each(|(e, _)| commands.entity(e).despawn());
}
