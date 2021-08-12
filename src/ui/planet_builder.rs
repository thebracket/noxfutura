use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Pos2},
    EguiContext,
};
use super::{BackgroundImage, UiResources};
use bevy_tilemap::prelude::*;

pub struct WorldgenTilemap;
pub struct WorldgenStatus {
    built_map: bool
}

pub fn planet_builder_menu(
    egui_context: ResMut<EguiContext>,
) {
    egui::Window::new("Planet Builder")
        .title_bar(true)
        .fixed_pos(Pos2::new(25.0, 25.0))
        .show(egui_context.ctx(), |ui| {
            ui.label("Magratheans on Line 1");
        });
}

pub fn resume_planet_builder_menu(
    mut commands: Commands,
    mut query: Query<(Entity, &TextureAtlasSprite, &BackgroundImage)>,
    handles: Res<UiResources>,

) {
    // Background image
    /*for (e, _, _) in query.iter_mut() {
        commands.entity(e).despawn();
    }*/

    println!("Building tile map components");

    commands.insert_resource(WorldgenStatus{ built_map: false });

    // Tile map
    let tilemap = Tilemap::builder()
        .auto_chunk()
        .topology(GridTopology::Square)
        .dimensions(1, 1)
        .chunk_dimensions(128, 128, 1)
        .texture_dimensions(8, 8)
        .z_layers(1)
        .texture_atlas(handles.worldgen_tiles.clone())
        .finish()
        .unwrap();

    let tilemap_components = TilemapBundle {
        tilemap,
        visible: Visible {
            is_visible: true,
            is_transparent: true,
        },
        transform: Default::default(),
        global_transform: Default::default(),
    };

    commands
        .spawn()
        .insert_bundle(tilemap_components);
}

pub fn construct_planet_builder_menu(
    mut query: Query<&mut Tilemap>,
    mut status: ResMut<WorldgenStatus>,
) {
    if status.built_map {
        return;
    }

    println!("Filling the map");
    for mut map in query.iter_mut() {
        println!("Found a map");

        let mut tiles = Vec::new();
        for y in -64..64 {
            for x in -64..64 {
                let tile = Tile {
                    point: (x, y),
                    sprite_index: 0,
                    ..Default::default()
                };
                tiles.push(tile);
            }
        }
        map.insert_tiles(tiles).unwrap();

        map.spawn_chunk((0, 0)).unwrap();
    }
    status.built_map = true;
}