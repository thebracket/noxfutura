use crate::planet::{Region, TileType};
use crate::utils::{ground_z, rex::*, mapidx};
use bracket_geometry::prelude::*;
use std::fs::File;
use crate::raws::get_material_by_tag;

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
                let mx = x-5+crash_site.x as usize;
                let my = y-11+crash_site.y as usize;
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
                            _ => println!("No decoder for glyph {} in spaceship", glyph.ch)
                        }
                    }
                }
            }
        }
    }
}

fn add_construction(region: &mut Region, x: usize, y: usize, z: usize, name: &str, solid:bool) {
    let plasteel = get_material_by_tag("Plasteel").unwrap();
    let idx = mapidx(x, y, z);
    match name {
        "ship_wall" => {
            region.tile_types[idx] = TileType::Solid;
            region.material_idx[idx] = plasteel;
        }
        "ship_window" => {
            region.tile_types[idx] = TileType::Solid;
            region.material_idx[idx] = plasteel;
        }
        "ship_floor" => {
            region.tile_types[idx] = TileType::Floor;
            region.material_idx[idx] = plasteel;
        }
        _ => {
            println!("Warning: No decoder for {}", name);
        }
    }
}