use crate::planet::{Region, TileType, StairsType};
use crate::raws::get_material_by_tag;
use crate::utils::{ground_z, mapidx, rex::*};
use bracket_geometry::prelude::*;
use std::fs::File;

fn load_ship() -> XpFile {
    let mut f = File::open("resources/rex/spaceship.xp").unwrap();
    XpFile::read(&mut f).unwrap()
}

pub fn build_escape_pod(region: &mut Region, crash_site: &Point) {
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
                            219 => add_construction(region, mx, my, mz, "ship_wall", true),
                            87 => add_construction(region, mx, my, mz, "ship_window", true),
                            176 => add_construction(region, mx, my, mz, "ship_floor", true),
                            88 => add_construction(region, mx, my, mz, "ship_updown", true),
                            60 => add_construction(region, mx, my, mz, "ship_up", true),
                            62 => add_construction(region, mx, my, mz, "ship_down", true),
                            178 => add_construction(region, mx, my, mz, "solar_panel", true),
                            241 => add_construction(region, mx, my, mz, "battery", true),
                            48 => add_construction(region, mx, my, mz, "cryo_bed", true),
                            236 => add_construction(region, mx, my, mz, "storage_locker", true),
                            67 => add_construction(region, mx, my, mz, "cordex", true),
                            243 => add_construction(region, mx, my, mz, "ship_defense_turret", true),
                            251 => add_construction(region, mx, my, mz, "small_replicator", true),
                            232 => add_construction(region, mx, my, mz, "rtg", true),
                            197 => add_construction(region, mx, my, mz, "ship_door", true),
                            76 => add_construction(region, mx, my, mz, "ship_lamp", true),
                            _ => println!("No decoder for glyph {} in spaceship", glyph.ch),
                        }
                    }
                }
            }
        }
    }
}

fn add_construction(region: &mut Region, x: usize, y: usize, z: usize, name: &str, solid: bool) {
    let plasteel = get_material_by_tag("Plasteel").unwrap();
    let idx = mapidx(x, y, z);
    region.tile_types[idx] = TileType::Floor;
    region.material_idx[idx] = plasteel;
    region.vegetation_type_id[idx] = None;

    match name {
        "ship_wall" => {
            region.tile_types[idx] = TileType::Solid;
        }
        "ship_window" => {
            region.tile_types[idx] = TileType::Window;
        }
        "ship_floor" => {}
        "ship_updown" => region.tile_types[idx] = TileType::Stairs{direction: StairsType::UpDown},
        "ship_up" => region.tile_types[idx] = TileType::Stairs{direction: StairsType::Up},
        "ship_down" => region.tile_types[idx] = TileType::Stairs{direction: StairsType::Down},
        "solar_panel" => add_building(region, "solar_panel", x, y, z),
        "battery" => add_building(region, "battery", x, y, z),
        "cryo_bed" => add_building(region, "cryo_bed", x, y, z),
        "storage_locker" => add_building(region, "storage_locker", x, y, z),
        "cordex" => add_building(region, "cordex", x, y, z),
        "ship_defense_turret" => add_building(region, "ship_defense_turret", x, y, z),
        "small_replicator" => add_building(region, "small_replicator", x, y, z),
        "rtg" => add_building(region, "rtg", x, y, z),
        "ship_door" => add_building(region, "ship_door", x, y, z),
        "ship_lamp" => add_building(region, "ship_lamp", x, y, z),
        _ => {
            println!("Warning: No decoder for {}", name);
        }
    }
}

fn add_building(region: &mut Region, tag: &str, x:usize, y:usize, z:usize) {

}