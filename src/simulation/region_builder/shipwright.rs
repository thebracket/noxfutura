use bevy::math::Vec3;

use crate::components::{PlanetLocation, Position};
use crate::{
    asset_handlers::rex::XpFile,
    simulation::{
        mapidx,
        spawner::spawn_raws_entity,
        terrain::{ground_z, submit_change_batch, ChangeRequest, MapChangeBatch, StairsType},
        REGION_HEIGHT, REGION_WIDTH,
    },
};
use std::fs::File;

fn load_ship() -> XpFile {
    let mut f = File::open("assets/rex/spaceship.xp").unwrap();
    XpFile::read(&mut f).unwrap()
}

pub fn build_escape_pod(region_id: PlanetLocation) {
    let mut changes = MapChangeBatch::new(region_id);
    let crash_x = REGION_WIDTH / 2;
    let crash_y = REGION_HEIGHT / 2;
    let ship = load_ship();
    let z = ground_z(region_id, crash_x, crash_y) - 2;

    let crash_v3 = Vec3::new(crash_x as f32, crash_y as f32, z as f32);
    for x in crash_x - 10..crash_x + 10 {
        for y in crash_y - 10..crash_y + 10 {
            let site_v3 = Vec3::new(x as f32, y as f32, z as f32 + 1.0);
            if crash_v3.distance(site_v3) < 8.0 {
                for z in z + 2..z + 7 {
                    changes.enqueue_change(ChangeRequest::EmptyTile {
                        idx: mapidx(x, y, z),
                    });
                }
            }
        }
    }

    let plasteel = crate::raws::RAWS.read().materials.find_by_name("Plasteel");

    for (i, layer) in ship.layers.iter().enumerate() {
        for y in 0..layer.height {
            for x in 0..layer.width {
                let mx = (x as i32 - 5 + crash_x as i32) as usize;
                let my = (y as i32 - 11 + crash_y as i32) as usize;
                let mz = z + i;
                let position = Position::with_tile_coords(region_id, mx, my, mz);
                let tile_idx = position.to_tile_index();

                let glyph = layer.get(x, y);
                if let Some(glyph) = glyph {
                    changes.enqueue_change(ChangeRequest::RevealTile { idx: tile_idx });
                    changes.enqueue_change(ChangeRequest::NoVegetation { idx: tile_idx });
                    match glyph.ch {
                        32 => {} // Ignore space
                        48 => {
                            changes.enqueue_change(ChangeRequest::Floor {
                                idx: tile_idx,
                                material: plasteel,
                            });
                            spawn_raws_entity(position, "cryo_bed");
                        }
                        60 => {
                            changes.enqueue_change(ChangeRequest::Stairs {
                                idx: tile_idx,
                                material: plasteel,
                                direction: StairsType::Up,
                            });
                            spawn_raws_entity(position, "stairs_up");
                        }
                        62 => {
                            changes.enqueue_change(ChangeRequest::Stairs {
                                idx: tile_idx,
                                material: plasteel,
                                direction: StairsType::Down,
                            });
                            spawn_raws_entity(position, "stairs_down");
                        }
                        67 => {
                            changes.enqueue_change(ChangeRequest::Floor {
                                idx: tile_idx,
                                material: plasteel,
                            });
                            spawn_raws_entity(position.offset(-1, -1, 0), "cordex");
                        }
                        76 => {
                            changes.enqueue_change(ChangeRequest::Floor {
                                idx: tile_idx,
                                material: plasteel,
                            });
                            spawn_raws_entity(position, "ship_lamp");
                        }
                        // Ship window - needs work
                        87 => changes.enqueue_change(ChangeRequest::SolidTile {
                            idx: tile_idx,
                            material: plasteel,
                        }),
                        88 => {
                            changes.enqueue_change(ChangeRequest::Stairs {
                                idx: tile_idx,
                                material: plasteel,
                                direction: StairsType::UpDown,
                            });
                            spawn_raws_entity(position, "stairs_updown");
                        }
                        176 => changes.enqueue_change(ChangeRequest::Floor {
                            idx: tile_idx,
                            material: plasteel,
                        }),
                        178 => {
                            changes.enqueue_change(ChangeRequest::Floor {
                                idx: tile_idx,
                                material: plasteel,
                            });
                            spawn_raws_entity(position, "solar_panel");
                        }
                        197 => {
                            changes.enqueue_change(ChangeRequest::Floor {
                                idx: tile_idx,
                                material: plasteel,
                            });
                            //spawn_raws_entity(region_id, tile_idx, "energydoor-open");
                        }
                        219 => changes.enqueue_change(ChangeRequest::SolidTile {
                            idx: tile_idx,
                            material: plasteel,
                        }),
                        232 => {
                            changes.enqueue_change(ChangeRequest::Floor {
                                idx: tile_idx,
                                material: plasteel,
                            });
                            spawn_raws_entity(position, "rtg");
                        }
                        236 => {
                            changes.enqueue_change(ChangeRequest::Floor {
                                idx: tile_idx,
                                material: plasteel,
                            });
                            spawn_raws_entity(position, "storage_locker");
                        }
                        241 => {
                            changes.enqueue_change(ChangeRequest::Floor {
                                idx: tile_idx,
                                material: plasteel,
                            });
                            spawn_raws_entity(position, "battery");
                        }
                        243 => {
                            changes.enqueue_change(ChangeRequest::Floor {
                                idx: tile_idx,
                                material: plasteel,
                            });
                            spawn_raws_entity(position, "ship_defense_turret");
                        }
                        251 => {
                            changes.enqueue_change(ChangeRequest::Floor {
                                idx: tile_idx,
                                material: plasteel,
                            });
                            spawn_raws_entity(position, "small_replicator");
                        }
                        _ => {
                            changes.enqueue_change(ChangeRequest::EmptyTile { idx: tile_idx });
                            println!("No decoder for glyph {} in spaceship", glyph.ch);
                        }
                    }
                }
            }
        }
    }
    submit_change_batch(changes);
}
