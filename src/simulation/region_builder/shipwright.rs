use bevy::math::Vec3;

use crate::{
    rex::XpFile,
    simulation::{
        mapidx,
        terrain::{ground_z, submit_change_batch, ChangeRequest, MapChangeBatch, PlanetLocation},
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
    for x in crash_x - 10 .. crash_x + 10 {
        for y in crash_y - 10 .. crash_y + 10 {
            let site_v3 = Vec3::new(x as f32, y as f32, z as f32 + 1.0);
            if crash_v3.distance(site_v3) < 8.0 {
                for z in z+2 .. z+7 {
                    changes.enqueue_change(ChangeRequest::EmptyTile{idx: mapidx(x, y, z)});
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
                let tile_idx = mapidx(mx as usize, my as usize, mz);

                let glyph = layer.get(x, y);
                if let Some(glyph) = glyph {
                    changes.enqueue_change(ChangeRequest::RevealTile { idx: tile_idx });
                    match glyph.ch {
                        32 => {} // Ignore space
                        176 => changes.enqueue_change(ChangeRequest::Floor{
                            idx: tile_idx,
                            material: plasteel,
                        }),
                        219 => changes.enqueue_change(ChangeRequest::SolidTile {
                            idx: tile_idx,
                            material: plasteel,
                        }),
                        _ => {
                            changes.enqueue_change(ChangeRequest::EmptyTile{idx: tile_idx});
                            println!("No decoder for glyph {} in spaceship", glyph.ch);
                        }
                    }
                }
            }
        }
    }
    submit_change_batch(changes);
}
