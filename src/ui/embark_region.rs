use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Pos2},
    EguiContext,
};

use crate::simulation::{region_builder::RegionBuilder, REGION_HEIGHT, REGION_WIDTH};

use super::EmbarkResources;

pub struct RegionGenUi;

pub fn embark_region_menu(egui_context: ResMut<EguiContext>, mut builder: ResMut<RegionBuilder>) {
    builder.start();

    egui::Window::new("Building Embark Region")
        .title_bar(true)
        .fixed_pos(Pos2::new(10.0, 10.0))
        .show(egui_context.ctx(), |ui| {
            ui.label(builder.status());
        });
}

pub fn resume_embark_region(mut commands: Commands, embark: Res<EmbarkResources>) {
    // Camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(
                (REGION_WIDTH as f32 * 1.25) + 2.0,
                REGION_HEIGHT as f32 * 1.25,
                256.0,
            )
            .looking_at(Vec3::ZERO, Vec3::Z),
            ..Default::default()
        })
        .insert(RegionGenUi {});

    // Region build link
    let rb = RegionBuilder::new(embark.planet.clone(), embark.tile_x, embark.tile_y);
    commands.insert_resource(rb);
}

pub fn embark_region_exit(mut commands: Commands, q: Query<(Entity, &RegionGenUi)>) {
    q.iter().for_each(|(e, _)| commands.entity(e).despawn());
}
