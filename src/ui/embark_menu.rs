use super::{BackgroundImage, UiCamera, UiResources};
use crate::{
    simulation::{planet_idx, Planet, WORLD_HEIGHT, WORLD_WIDTH},
    AppState,
};
use bevy::prelude::*;
use bevy::{input::mouse::MouseButtonInput, math::ivec3};
use bevy_egui::{
    egui::{self, Pos2},
    EguiContext,
};
use bevy_simple_tilemap::prelude::*;

pub struct EmbarkResources {
    pub planet: Planet,
    pub tile_x: usize,
    pub tile_y: usize,
}

pub struct EmbarkGrid;

pub fn embark_menu(
    egui_context: ResMut<EguiContext>,
    wnds: Res<Windows>,
    q_camera: Query<&Transform, With<UiCamera>>,
    mut embark: ResMut<EmbarkResources>,
    mut mouse_button_event_reader: EventReader<MouseButtonInput>,
    mut state: ResMut<State<AppState>>,
) {
    // Mouse Picking
    let mut tile_x = 0;
    let mut tile_y = 0;
    let mut description = String::new();
    // get the primary window
    let wnd = wnds.get_primary().unwrap();

    // check if the cursor is in the primary window
    if let Some(pos) = wnd.cursor_position() {
        // get the size of the window
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = pos - size / 2.0;

        // assuming there is exactly one main camera entity, so this is OK
        let camera_transform = q_camera.single().unwrap();

        // apply the camera transform
        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        //eprintln!("World coords: {}/{}", pos_wld.x, pos_wld.y);
        let width = WORLD_WIDTH as f32 * 8.0;
        let height = WORLD_HEIGHT as f32 * 8.0;
        if pos_wld.y > -(height / 2.0) && pos_wld.y < height / 2.0 {
            if pos_wld.x > -(width / 2.0) && pos_wld.x < width / 2.0 {
                tile_x = ((pos_wld.x + (width / 2.0)) / 8.0) as i32;
                tile_y = ((pos_wld.y + (height / 2.0)) / 8.0) as i32;

                let pidx = planet_idx(tile_x as usize, tile_y as usize);
                let lb = &embark.planet.landblocks[pidx];
                let bidx = lb.biome_idx;
                description = format!("{}.\n Avg Altitude: {}.\n Rainfall: {}mm.\n Variance: {}\nAvg Temperature: {} C",
                    crate::raws::RAWS.read().biomes.areas[bidx].name,
                    lb.height,
                    lb.rainfall_mm,
                    lb.variance,
                    lb.temperature_c,
                );
            }
        }
    }

    if tile_x != 0 && tile_y != 0 {
        for event in mouse_button_event_reader.iter() {
            if event.state.is_pressed() && event.button == MouseButton::Left {
                embark.tile_x = tile_x as usize;
                embark.tile_y = tile_y as usize;
                state.set(AppState::EmbarkBuildRegion).unwrap();
            }
        }
    }

    egui::Window::new("Prepare to Evacuate the Colony Ship")
        .title_bar(true)
        .fixed_pos(Pos2::new(10.0, 10.0))
        .show(egui_context.ctx(), |ui| {
            if tile_x != 0 && tile_y != 0 {
                ui.label("Select escape pod target");
                ui.label(format!("Tile: {}, {}", tile_x, tile_y));
                ui.label(description);
            }
        });
}

pub fn resume_embark_menu(mut commands: Commands, ui: Res<UiResources>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(UiCamera {})
        .insert(EmbarkGrid {});

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: ui.backgrounds.clone(),
            sprite: TextureAtlasSprite::new(1),
            ..Default::default()
        })
        .insert(BackgroundImage {})
        .insert(EmbarkGrid {});

    let planet = crate::simulation::load_planet();
    let mut tiles: Vec<(IVec3, Option<Tile>)> = Vec::new();
    for y in 0..WORLD_HEIGHT as i32 {
        for x in 0..WORLD_WIDTH as i32 {
            let pidx = planet_idx(x as usize, y as usize);
            let biome_idx = planet.landblocks[pidx].biome_idx;
            let tile_index = crate::raws::RAWS.read().biomes.areas[biome_idx].embark_tile;
            let tx = x - WORLD_WIDTH as i32 / 2;
            let ty = y - WORLD_HEIGHT as i32 / 2;
            tiles.push((
                ivec3(tx, ty, 0),
                Some(Tile {
                    sprite_index: tile_index as u32,
                    ..Default::default()
                }),
            ));
        }
    }

    let mut tilemap = TileMap::default();
    tilemap.set_tiles(tiles);

    // Set up tilemap
    let tilemap_bundle = TileMapBundle {
        tilemap,
        texture_atlas: ui.embark_tiles.clone(),
        transform: Transform {
            scale: Vec3::splat(1.0),
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    };

    commands.spawn_bundle(tilemap_bundle).insert(EmbarkGrid {});
    commands.insert_resource(EmbarkResources {
        planet,
        tile_x: 0,
        tile_y: 0,
    });
}

pub fn embark_exit(mut commands: Commands, q3: Query<Entity>) {
    // The bundle doesn't correctly tag the tilemap, which is a pain.
    q3.iter()
        .for_each(|entity| commands.entity(entity).despawn());
}
