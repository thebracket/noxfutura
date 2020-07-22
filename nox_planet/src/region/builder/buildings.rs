use crate::{ground_z, rex::*, Region, StairsType, TileType};
use bracket_geometry::prelude::*;
use legion::prelude::*;
use nox_components::*;
use nox_raws::get_material_by_tag;
use nox_spatial::mapidx;
use std::fs::File;

fn load_ship() -> XpFile {
    let mut f = File::open("resources/rex/spaceship.xp").unwrap();
    XpFile::read(&mut f).unwrap()
}

pub fn build_escape_pod(region: &mut Region, crash_site: &Point, ecs: &mut World) -> usize {
    let z = ground_z(region, crash_site.x as usize, crash_site.y as usize) - 2;
    let ship = load_ship();

    for (i, layer) in ship.layers.iter().enumerate() {
        for y in 0..layer.height {
            for x in 0..layer.width {
                let mx = x - 5 + crash_site.x as usize;
                let my = y - 11 + crash_site.y as usize;
                let mz = z + i;
                let tile_idx = mapidx(mx as usize, my as usize, mz);

                let glyph = layer.get(x, y);
                if let Some(glyph) = glyph {
                    if glyph.ch != 32 {
                        region.revealed[tile_idx] = true;
                        match glyph.ch {
                            219 => add_construction(region, mx, my, mz, "ship_wall", true, ecs),
                            87 => add_construction(region, mx, my, mz, "ship_window", true, ecs),
                            176 => add_construction(region, mx, my, mz, "ship_floor", false, ecs),
                            88 => add_construction(region, mx, my, mz, "ship_updown", false, ecs),
                            60 => add_construction(region, mx, my, mz, "ship_up", false, ecs),
                            62 => add_construction(region, mx, my, mz, "ship_down", false, ecs),
                            178 => add_construction(region, mx, my, mz, "solar_panel", true, ecs),
                            241 => add_construction(region, mx, my, mz, "battery", false, ecs),
                            48 => add_construction(region, mx, my, mz, "cryo_bed", false, ecs),
                            236 => {
                                add_construction(region, mx, my, mz, "storage_locker", false, ecs)
                            }
                            67 => add_construction(region, mx, my, mz, "cordex", false, ecs),
                            243 => add_construction(
                                region,
                                mx,
                                my,
                                mz,
                                "ship_defense_turret",
                                true,
                                ecs,
                            ),
                            251 => {
                                add_construction(region, mx, my, mz, "small_replicator", false, ecs)
                            }
                            232 => add_construction(region, mx, my, mz, "rtg", false, ecs),
                            197 => add_construction(region, mx, my, mz, "ship_door", false, ecs),
                            76 => add_construction(region, mx, my, mz, "ship_lamp", false, ecs),
                            _ => println!("No decoder for glyph {} in spaceship", glyph.ch),
                        }
                    }
                }
            }
        }
    }
    z
}

fn add_construction(
    region: &mut Region,
    x: usize,
    y: usize,
    z: usize,
    name: &str,
    solid: bool,
    ecs: &mut World,
) {
    let plasteel = get_material_by_tag("Plasteel").unwrap();
    let idx = mapidx(x, y, z);
    region.tile_types[idx] = TileType::Floor{ plant: None };
    region.material_idx[idx] = plasteel;
    region.set_flag(idx, Region::CONSTRUCTED);
    if solid {
        region.set_flag(idx, Region::SOLID);
    }

    // Remove any vegetation
    let veg_list_delete = <Read<Position>>::query()
        .filter(tag::<Vegetation>())
        .iter_entities_mut(ecs)
        .filter(|(_, pos)| pos.exact_position(x, y, z))
        .map(|(entity, _)| entity)
        .collect::<Vec<Entity>>();
    veg_list_delete.iter().for_each(|e| {
        ecs.delete(*e);
    });

    match name {
        "ship_wall" => {
            region.tile_types[idx] = TileType::Solid;
        }
        "ship_window" => {
            region.tile_types[idx] = TileType::Solid;
        }
        "ship_floor" => {}
        "ship_updown" => {
            region.tile_types[idx] = TileType::Stairs {
                direction: StairsType::UpDown,
            }
        }
        "ship_up" => {
            region.tile_types[idx] = TileType::Stairs {
                direction: StairsType::Up,
            }
        }
        "ship_down" => {
            region.tile_types[idx] = TileType::Stairs {
                direction: StairsType::Down,
            }
        }
        "solar_panel" => {
            add_building(region, "solar_panel", x, y, z, ecs);
        }
        "battery" => {
            add_building(region, "battery", x, y, z, ecs);
        }
        "cryo_bed" => {
            add_building(region, "cryo_bed", x, y, z, ecs);
        }
        "storage_locker" => {
            let storage_id = add_building(region, "storage_locker", x, y, z, ecs);
            crate::spawner::spawn_item_in_container(
                ecs,
                "personal_survival_shelter_kit",
                storage_id,
                region,
            );
            crate::spawner::spawn_item_in_container(
                ecs,
                "personal_survival_shelter_kit",
                storage_id,
                region,
            );
            crate::spawner::spawn_item_in_container(ecs, "camp_fire_kit", storage_id, region);
            crate::spawner::spawn_item_in_container(ecs, "personal_survival_shelter_kit", storage_id, region);
            crate::spawner::spawn_item_in_container(ecs, "fire_axe", storage_id, region);
            crate::spawner::spawn_item_in_container(ecs, "pickaxe", storage_id, region);
            crate::spawner::spawn_item_in_container(ecs, "hoe", storage_id, region);
        }
        "cordex" => {
            add_building(region, "cordex", x, y, z, ecs);
        }
        "ship_defense_turret" => {
            add_building(region, "ship_defense_turret", x, y, z, ecs);
        }
        "small_replicator" => {
            add_building(region, "small_replicator", x, y, z, ecs);
        }
        "rtg" => {
            add_building(region, "rtg", x, y, z, ecs);
        }
        "ship_door" => {
            add_building(region, "ship_door", x, y, z, ecs);
        }
        "ship_lamp" => {
            add_building(region, "ship_lamp", x, y, z, ecs);
        }
        _ => {
            println!("Warning: No decoder for {}", name);
        }
    }
}

fn add_building(
    region: &mut Region,
    tag: &str,
    x: usize,
    y: usize,
    z: usize,
    ecs: &mut World,
) -> usize {
    crate::spawner::spawn_building(ecs, tag, x, y, z, region.world_idx)
}
